/* FP:lib.rs-0001 */ // Low-level Rust lexer.
/* FP:lib.rs-0002 */ //
/* FP:lib.rs-0003 */ // The idea with `rustc_lexer` is to make a reusable library,
/* FP:lib.rs-0004 */ // by separating out pure lexing and rustc-specific concerns, like spans,
/* FP:lib.rs-0005 */ // error reporting, and interning. So, rustc_lexer operates directly on `&str`,
/* FP:lib.rs-0006 */ // produces simple tokens which are a pair of type-tag and a bit of original text,
/* FP:lib.rs-0007 */ // and does not report errors, instead storing them as flags on the token.
/* FP:lib.rs-0008 */ //
/* FP:lib.rs-0009 */ // Tokens produced by this lexer are not yet ready for parsing the Rust syntax.
/* FP:lib.rs-0010 */ // For that see [`crate::rustc_parse::lexer`], which converts this basic token stream
/* FP:lib.rs-0011 */ // into wide tokens used by actual parser.
/* FP:lib.rs-0012 */ //
/* FP:lib.rs-0013 */ // The purpose of this crate is to convert raw sources into a labeled sequence
/* FP:lib.rs-0014 */ // of well-known token types, so building an actual Rust token stream will
/* FP:lib.rs-0015 */ // be easier.
/* FP:lib.rs-0016 */ //
/* FP:lib.rs-0017 */ // The main entity of this crate is the [`TokenKind`] enum which represents common
/* FP:lib.rs-0018 */ // lexeme types.
/* FP:lib.rs-0019 */ //
/* FP:lib.rs-0020 */ // [`crate::rustc_parse::lexer`]: ../rustc_parse/lexer/index.html
/* FP:lib.rs-0021 */ 
/* FP:lib.rs-0022 */ // tidy-alphabetical-start
/* FP:lib.rs-0023 */ // We want to be able to build this crate with a stable compiler,
/* FP:lib.rs-0024 */ // so no `#[feature]` attributes should be added.
/* FP:lib.rs-0025 */ #[deny(unstable_features)]
/* FP:lib.rs-0026 */ // tidy-alphabetical-end
/* FP:lib.rs-0027 */ 
/* FP:lib.rs-0029 */ 
/* FP:lib.rs-0030 */ #[cfg(test)]
/* FP:lib.rs-0032 */ 
/* FP:lib.rs-0033 */ use LiteralKind::*;
/* FP:lib.rs-0034 */ use TokenKind::*;
/* FP:lib.rs-0035 */ use cursor::EOF_CHAR;
/* FP:lib.rs-0036 */ pub use cursor::{Cursor, FrontmatterAllowed};
/* FP:lib.rs-0037 */ use unicode_properties::UnicodeEmoji;
/* FP:lib.rs-0038 */ pub use unicode_xid::UNICODE_VERSION as UNICODE_XID_VERSION;
/* FP:lib.rs-0039 */ 
/* FP:lib.rs-0040 */ /// Parsed token.
/* FP:lib.rs-0041 */ /// It doesn't contain information about data that has been parsed,
/* FP:lib.rs-0042 */ /// only the type of the token and its size.
/* FP:lib.rs-0043 */ #[derive(Debug)]
/* FP:lib.rs-0044 */ pub struct Token {
/* FP:lib.rs-0045 */     pub kind: TokenKind,
/* FP:lib.rs-0046 */     pub len: u32,
/* FP:lib.rs-0047 */ }
/* FP:lib.rs-0048 */ 
/* FP:lib.rs-0049 */ impl Token {
/* FP:lib.rs-0050 */     fn new(kind: TokenKind, len: u32) -> Token {
/* FP:lib.rs-0051 */         Token { kind, len }
/* FP:lib.rs-0052 */     }
/* FP:lib.rs-0053 */ }
/* FP:lib.rs-0054 */ 
/* FP:lib.rs-0055 */ /// Enum representing common lexeme types.
/* FP:lib.rs-0056 */ #[derive(Clone, Copy, Debug, PartialEq, Eq)]
/* FP:lib.rs-0057 */ pub enum TokenKind {
/* FP:lib.rs-0058 */     /// A line comment, e.g. `// comment`.
/* FP:lib.rs-0059 */     LineComment {
/* FP:lib.rs-0060 */         doc_style: Option<DocStyle>,
/* FP:lib.rs-0061 */     },
/* FP:lib.rs-0062 */ 
/* FP:lib.rs-0063 */     /// A block comment, e.g. `/* block comment */`.
/* FP:lib.rs-0064 */     ///
/* FP:lib.rs-0065 */     /// Block comments can be recursive, so a sequence like `/* /* */`
/* FP:lib.rs-0066 */     /// will not be considered terminated and will result in a parsing error.
/* FP:lib.rs-0067 */     BlockComment {
/* FP:lib.rs-0068 */         doc_style: Option<DocStyle>,
/* FP:lib.rs-0069 */         terminated: bool,
/* FP:lib.rs-0070 */     },
/* FP:lib.rs-0071 */ 
/* FP:lib.rs-0072 */     /// Any whitespace character sequence.
/* FP:lib.rs-0073 */     Whitespace,
/* FP:lib.rs-0074 */ 
/* FP:lib.rs-0075 */     Frontmatter {
/* FP:lib.rs-0076 */         has_invalid_preceding_whitespace: bool,
/* FP:lib.rs-0077 */         invalid_infostring: bool,
/* FP:lib.rs-0078 */     },
/* FP:lib.rs-0079 */ 
/* FP:lib.rs-0080 */     /// An identifier or keyword, e.g. `ident` or `continue`.
/* FP:lib.rs-0081 */     Ident,
/* FP:lib.rs-0082 */ 
/* FP:lib.rs-0083 */     /// An identifier that is invalid because it contains emoji.
/* FP:lib.rs-0084 */     InvalidIdent,
/* FP:lib.rs-0085 */ 
/* FP:lib.rs-0086 */     /// A raw identifier, e.g. "r#ident".
/* FP:lib.rs-0087 */     RawIdent,
/* FP:lib.rs-0088 */ 
/* FP:lib.rs-0089 */     /// An unknown literal prefix, like `foo#`, `foo'`, `foo"`. Excludes
/* FP:lib.rs-0090 */     /// literal prefixes that contain emoji, which are considered "invalid".
/* FP:lib.rs-0091 */     ///
/* FP:lib.rs-0092 */     /// Note that only the
/* FP:lib.rs-0093 */     /// prefix (`foo`) is included in the token, not the separator (which is
/* FP:lib.rs-0094 */     /// lexed as its own distinct token). In Rust 2021 and later, reserved
/* FP:lib.rs-0095 */     /// prefixes are reported as errors; in earlier editions, they result in a
/* FP:lib.rs-0096 */     /// (allowed by default) lint, and are treated as regular identifier
/* FP:lib.rs-0097 */     /// tokens.
/* FP:lib.rs-0098 */     UnknownPrefix,
/* FP:lib.rs-0099 */ 
/* FP:lib.rs-0100 */     /// An unknown prefix in a lifetime, like `'foo#`.
/* FP:lib.rs-0101 */     ///
/* FP:lib.rs-0102 */     /// Like `UnknownPrefix`, only the `'` and prefix are included in the token
/* FP:lib.rs-0103 */     /// and not the separator.
/* FP:lib.rs-0104 */     UnknownPrefixLifetime,
/* FP:lib.rs-0105 */ 
/* FP:lib.rs-0106 */     /// A raw lifetime, e.g. `'r#foo`. In edition < 2021 it will be split into
/* FP:lib.rs-0107 */     /// several tokens: `'r` and `#` and `foo`.
/* FP:lib.rs-0108 */     RawLifetime,
/* FP:lib.rs-0109 */ 
/* FP:lib.rs-0110 */     /// Guarded string literal prefix: `#"` or `##`.
/* FP:lib.rs-0111 */     ///
/* FP:lib.rs-0112 */     /// Used for reserving "guarded strings" (RFC 3598) in edition 2024.
/* FP:lib.rs-0113 */     /// Split into the component tokens on older editions.
/* FP:lib.rs-0114 */     GuardedStrPrefix,
/* FP:lib.rs-0115 */ 
/* FP:lib.rs-0116 */     /// Literals, e.g. `12u8`, `1.0e-40`, `b"123"`. Note that `_` is an invalid
/* FP:lib.rs-0117 */     /// suffix, but may be present here on string and float literals. Users of
/* FP:lib.rs-0118 */     /// this type will need to check for and reject that case.
/* FP:lib.rs-0119 */     ///
/* FP:lib.rs-0120 */     /// See [LiteralKind] for more details.
/* FP:lib.rs-0121 */     Literal {
/* FP:lib.rs-0122 */         kind: LiteralKind,
/* FP:lib.rs-0123 */         suffix_start: u32,
/* FP:lib.rs-0124 */     },
/* FP:lib.rs-0125 */ 
/* FP:lib.rs-0126 */     /// A lifetime, e.g. `'a`.
/* FP:lib.rs-0127 */     Lifetime {
/* FP:lib.rs-0128 */         starts_with_number: bool,
/* FP:lib.rs-0129 */     },
/* FP:lib.rs-0130 */ 
/* FP:lib.rs-0131 */     /// `;`
/* FP:lib.rs-0132 */     Semi,
/* FP:lib.rs-0133 */     /// `,`
/* FP:lib.rs-0134 */     Comma,
/* FP:lib.rs-0135 */     /// `.`
/* FP:lib.rs-0136 */     Dot,
/* FP:lib.rs-0137 */     /// `(`
/* FP:lib.rs-0138 */     OpenParen,
/* FP:lib.rs-0139 */     /// `)`
/* FP:lib.rs-0140 */     CloseParen,
/* FP:lib.rs-0141 */     /// `{`
/* FP:lib.rs-0142 */     OpenBrace,
/* FP:lib.rs-0143 */     /// `}`
/* FP:lib.rs-0144 */     CloseBrace,
/* FP:lib.rs-0145 */     /// `[`
/* FP:lib.rs-0146 */     OpenBracket,
/* FP:lib.rs-0147 */     /// `]`
/* FP:lib.rs-0148 */     CloseBracket,
/* FP:lib.rs-0149 */     /// `@`
/* FP:lib.rs-0150 */     At,
/* FP:lib.rs-0151 */     /// `#`
/* FP:lib.rs-0152 */     Pound,
/* FP:lib.rs-0153 */     /// `~`
/* FP:lib.rs-0154 */     Tilde,
/* FP:lib.rs-0155 */     /// `?`
/* FP:lib.rs-0156 */     Question,
/* FP:lib.rs-0157 */     /// `:`
/* FP:lib.rs-0158 */     Colon,
/* FP:lib.rs-0159 */     /// `$`
/* FP:lib.rs-0160 */     Dollar,
/* FP:lib.rs-0161 */     /// `=`
/* FP:lib.rs-0162 */     Eq,
/* FP:lib.rs-0163 */     /// `!`
/* FP:lib.rs-0164 */     Bang,
/* FP:lib.rs-0165 */     /// `<`
/* FP:lib.rs-0166 */     Lt,
/* FP:lib.rs-0167 */     /// `>`
/* FP:lib.rs-0168 */     Gt,
/* FP:lib.rs-0169 */     /// `-`
/* FP:lib.rs-0170 */     Minus,
/* FP:lib.rs-0171 */     /// `&`
/* FP:lib.rs-0172 */     And,
/* FP:lib.rs-0173 */     /// `|`
/* FP:lib.rs-0174 */     Or,
/* FP:lib.rs-0175 */     /// `+`
/* FP:lib.rs-0176 */     Plus,
/* FP:lib.rs-0177 */     /// `*`
/* FP:lib.rs-0178 */     Star,
/* FP:lib.rs-0179 */     /// `/`
/* FP:lib.rs-0180 */     Slash,
/* FP:lib.rs-0181 */     /// `^`
/* FP:lib.rs-0182 */     Caret,
/* FP:lib.rs-0183 */     /// `%`
/* FP:lib.rs-0184 */     Percent,
/* FP:lib.rs-0185 */ 
/* FP:lib.rs-0186 */     /// Unknown token, not expected by the lexer, e.g. "№"
/* FP:lib.rs-0187 */     Unknown,
/* FP:lib.rs-0188 */ 
/* FP:lib.rs-0189 */     /// End of input.
/* FP:lib.rs-0190 */     Eof,
/* FP:lib.rs-0191 */ }
/* FP:lib.rs-0192 */ 
/* FP:lib.rs-0193 */ #[derive(Clone, Copy, Debug, PartialEq, Eq)]
/* FP:lib.rs-0194 */ pub enum DocStyle {
/* FP:lib.rs-0195 */     Outer,
/* FP:lib.rs-0196 */     Inner,
/* FP:lib.rs-0197 */ }
/* FP:lib.rs-0198 */ 
/* FP:lib.rs-0199 */ /// Enum representing the literal types supported by the lexer.
/* FP:lib.rs-0200 */ ///
/* FP:lib.rs-0201 */ /// Note that the suffix is *not* considered when deciding the `LiteralKind` in
/* FP:lib.rs-0202 */ /// this type. This means that float literals like `1f32` are classified by this
/* FP:lib.rs-0203 */ /// type as `Int`. (Compare against `crate::rustc_ast::token::LitKind` and
/* FP:lib.rs-0204 */ /// `crate::rustc_ast::ast::LitKind`).
/* FP:lib.rs-0205 */ #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
/* FP:lib.rs-0206 */ pub enum LiteralKind {
/* FP:lib.rs-0207 */     /// `12_u8`, `0o100`, `0b120i99`, `1f32`.
/* FP:lib.rs-0208 */     Int { base: Base, empty_int: bool },
/* FP:lib.rs-0209 */     /// `12.34f32`, `1e3`, but not `1f32`.
/* FP:lib.rs-0210 */     Float { base: Base, empty_exponent: bool },
/* FP:lib.rs-0211 */     /// `'a'`, `'\\'`, `'''`, `';`
/* FP:lib.rs-0212 */     Char { terminated: bool },
/* FP:lib.rs-0213 */     /// `b'a'`, `b'\\'`, `b'''`, `b';`
/* FP:lib.rs-0214 */     Byte { terminated: bool },
/* FP:lib.rs-0215 */     /// `"abc"`, `"abc`
/* FP:lib.rs-0216 */     Str { terminated: bool },
/* FP:lib.rs-0217 */     /// `b"abc"`, `b"abc`
/* FP:lib.rs-0218 */     ByteStr { terminated: bool },
/* FP:lib.rs-0219 */     /// `c"abc"`, `c"abc`
/* FP:lib.rs-0220 */     CStr { terminated: bool },
/* FP:lib.rs-0221 */     /// `r"abc"`, `r#"abc"#`, `r####"ab"###"c"####`, `r#"a`. `None` indicates
/* FP:lib.rs-0222 */     /// an invalid literal.
/* FP:lib.rs-0223 */     RawStr { n_hashes: Option<u8> },
/* FP:lib.rs-0224 */     /// `br"abc"`, `br#"abc"#`, `br####"ab"###"c"####`, `br#"a`. `None`
/* FP:lib.rs-0225 */     /// indicates an invalid literal.
/* FP:lib.rs-0226 */     RawByteStr { n_hashes: Option<u8> },
/* FP:lib.rs-0227 */     /// `cr"abc"`, "cr#"abc"#", `cr#"a`. `None` indicates an invalid literal.
/* FP:lib.rs-0228 */     RawCStr { n_hashes: Option<u8> },
/* FP:lib.rs-0229 */ }
/* FP:lib.rs-0230 */ 
/* FP:lib.rs-0231 */ /// `#"abc"#`, `##"a"` (fewer closing), or even `#"a` (unterminated).
/* FP:lib.rs-0232 */ ///
/* FP:lib.rs-0233 */ /// Can capture fewer closing hashes than starting hashes,
/* FP:lib.rs-0234 */ /// for more efficient lexing and better backwards diagnostics.
/* FP:lib.rs-0235 */ #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
/* FP:lib.rs-0236 */ pub struct GuardedStr {
/* FP:lib.rs-0237 */     pub n_hashes: u32,
/* FP:lib.rs-0238 */     pub terminated: bool,
/* FP:lib.rs-0239 */     pub token_len: u32,
/* FP:lib.rs-0240 */ }
/* FP:lib.rs-0241 */ 
/* FP:lib.rs-0242 */ #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
/* FP:lib.rs-0243 */ pub enum RawStrError {
/* FP:lib.rs-0244 */     /// Non `#` characters exist between `r` and `"`, e.g. `r##~"abcde"##`
/* FP:lib.rs-0245 */     InvalidStarter { bad_char: char },
/* FP:lib.rs-0246 */     /// The string was not terminated, e.g. `r###"abcde"##`.
/* FP:lib.rs-0247 */     /// `possible_terminator_offset` is the number of characters after `r` or
/* FP:lib.rs-0248 */     /// `br` where they may have intended to terminate it.
/* FP:lib.rs-0249 */     NoTerminator { expected: u32, found: u32, possible_terminator_offset: Option<u32> },
/* FP:lib.rs-0250 */     /// More than 255 `#`s exist.
/* FP:lib.rs-0251 */     TooManyDelimiters { found: u32 },
/* FP:lib.rs-0252 */ }
/* FP:lib.rs-0253 */ 
/* FP:lib.rs-0254 */ /// Base of numeric literal encoding according to its prefix.
/* FP:lib.rs-0255 */ #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
/* FP:lib.rs-0256 */ pub enum Base {
/* FP:lib.rs-0257 */     /// Literal starts with "0b".
/* FP:lib.rs-0258 */     Binary = 2,
/* FP:lib.rs-0259 */     /// Literal starts with "0o".
/* FP:lib.rs-0260 */     Octal = 8,
/* FP:lib.rs-0261 */     /// Literal doesn't contain a prefix.
/* FP:lib.rs-0262 */     Decimal = 10,
/* FP:lib.rs-0263 */     /// Literal starts with "0x".
/* FP:lib.rs-0264 */     Hexadecimal = 16,
/* FP:lib.rs-0265 */ }
/* FP:lib.rs-0266 */ 
/* FP:lib.rs-0267 */ /// `rustc` allows files to have a shebang, e.g. "#!/usr/bin/rustrun",
/* FP:lib.rs-0268 */ /// but shebang isn't a part of rust syntax.
/* FP:lib.rs-0269 */ pub fn strip_shebang(input: &str) -> Option<usize> {
/* FP:lib.rs-0270 */     // Shebang must start with `#!` literally, without any preceding whitespace.
/* FP:lib.rs-0271 */     // For simplicity we consider any line starting with `#!` a shebang,
/* FP:lib.rs-0272 */     // regardless of restrictions put on shebangs by specific platforms.
/* FP:lib.rs-0273 */     if let Some(input_tail) = input.strip_prefix("#!") {
/* FP:lib.rs-0274 */         // Ok, this is a shebang but if the next non-whitespace token is `[`,
/* FP:lib.rs-0275 */         // then it may be valid Rust code, so consider it Rust code.
/* FP:lib.rs-0276 */         let next_non_whitespace_token =
/* FP:lib.rs-0277 */             tokenize(input_tail, FrontmatterAllowed::No).map(|tok| tok.kind).find(|tok| {
/* FP:lib.rs-0278 */                 !matches!(
/* FP:lib.rs-0279 */                     tok,
/* FP:lib.rs-0280 */                     TokenKind::Whitespace
/* FP:lib.rs-0281 */                         | TokenKind::LineComment { doc_style: None }
/* FP:lib.rs-0282 */                         | TokenKind::BlockComment { doc_style: None, .. }
/* FP:lib.rs-0283 */                 )
/* FP:lib.rs-0284 */             });
/* FP:lib.rs-0285 */         if next_non_whitespace_token != Some(TokenKind::OpenBracket) {
/* FP:lib.rs-0286 */             // No other choice than to consider this a shebang.
/* FP:lib.rs-0287 */             return Some(2 + input_tail.lines().next().unwrap_or_default().len());
/* FP:lib.rs-0288 */         }
/* FP:lib.rs-0289 */     }
/* FP:lib.rs-0290 */     None
/* FP:lib.rs-0291 */ }
/* FP:lib.rs-0292 */ 
/* FP:lib.rs-0293 */ /// Validates a raw string literal. Used for getting more information about a
/* FP:lib.rs-0294 */ /// problem with a `RawStr`/`RawByteStr` with a `None` field.
/* FP:lib.rs-0295 */ #[inline]
/* FP:lib.rs-0296 */ pub fn validate_raw_str(input: &str, prefix_len: u32) -> Result<(), RawStrError> {
/* FP:lib.rs-0297 */     debug_assert!(!input.is_empty());
/* FP:lib.rs-0298 */     let mut cursor = Cursor::new(input, FrontmatterAllowed::No);
/* FP:lib.rs-0299 */     // Move past the leading `r` or `br`.
/* FP:lib.rs-0300 */     for _ in 0..prefix_len {
/* FP:lib.rs-0301 */         cursor.bump().unwrap();
/* FP:lib.rs-0302 */     }
/* FP:lib.rs-0303 */     cursor.raw_double_quoted_string(prefix_len).map(|_| ())
/* FP:lib.rs-0304 */ }
/* FP:lib.rs-0305 */ 
/* FP:lib.rs-0306 */ /// Creates an iterator that produces tokens from the input string.
/* FP:lib.rs-0307 */ ///
/* FP:lib.rs-0308 */ /// When parsing a full Rust document,
/* FP:lib.rs-0309 */ /// first [`strip_shebang`] and then allow frontmatters with [`FrontmatterAllowed::Yes`].
/* FP:lib.rs-0310 */ ///
/* FP:lib.rs-0311 */ /// When tokenizing a slice of a document, be sure to disallow frontmatters with [`FrontmatterAllowed::No`]
/* FP:lib.rs-0312 */ pub fn tokenize(
/* FP:lib.rs-0313 */     input: &str,
/* FP:lib.rs-0314 */     frontmatter_allowed: FrontmatterAllowed,
/* FP:lib.rs-0315 */ ) -> impl Iterator<Item = Token> {
/* FP:lib.rs-0316 */     let mut cursor = Cursor::new(input, frontmatter_allowed);
/* FP:lib.rs-0317 */     std::iter::from_fn(move || {
/* FP:lib.rs-0318 */         let token = cursor.advance_token();
/* FP:lib.rs-0319 */         if token.kind != TokenKind::Eof { Some(token) } else { None }
/* FP:lib.rs-0320 */     })
/* FP:lib.rs-0321 */ }
/* FP:lib.rs-0322 */ 
/* FP:lib.rs-0323 */ /// True if `c` is considered a whitespace according to Rust language definition.
/* FP:lib.rs-0324 */ /// See [Rust language reference](https://doc.rust-lang.org/reference/whitespace.html)
/* FP:lib.rs-0325 */ /// for definitions of these classes.
/* FP:lib.rs-0326 */ pub fn is_whitespace(c: char) -> bool {
/* FP:lib.rs-0327 */     // This is Pattern_White_Space.
/* FP:lib.rs-0328 */     //
/* FP:lib.rs-0329 */     // Note that this set is stable (ie, it doesn't change with different
/* FP:lib.rs-0330 */     // Unicode versions), so it's ok to just hard-code the values.
/* FP:lib.rs-0331 */ 
/* FP:lib.rs-0332 */     matches!(
/* FP:lib.rs-0333 */         c,
/* FP:lib.rs-0334 */         // End-of-line characters
/* FP:lib.rs-0335 */         | '\u{000A}' // line feed (\n)
/* FP:lib.rs-0336 */         | '\u{000B}' // vertical tab
/* FP:lib.rs-0337 */         | '\u{000C}' // form feed
/* FP:lib.rs-0338 */         | '\u{000D}' // carriage return (\r)
/* FP:lib.rs-0339 */         | '\u{0085}' // next line (from latin1)
/* FP:lib.rs-0340 */         | '\u{2028}' // LINE SEPARATOR
/* FP:lib.rs-0341 */         | '\u{2029}' // PARAGRAPH SEPARATOR
/* FP:lib.rs-0342 */ 
/* FP:lib.rs-0343 */         // `Default_Ignorable_Code_Point` characters
/* FP:lib.rs-0344 */         | '\u{200E}' // LEFT-TO-RIGHT MARK
/* FP:lib.rs-0345 */         | '\u{200F}' // RIGHT-TO-LEFT MARK
/* FP:lib.rs-0346 */ 
/* FP:lib.rs-0347 */         // Horizontal space characters
/* FP:lib.rs-0348 */         | '\u{0009}'   // tab (\t)
/* FP:lib.rs-0349 */         | '\u{0020}' // space
/* FP:lib.rs-0350 */     )
/* FP:lib.rs-0351 */ }
/* FP:lib.rs-0352 */ 
/* FP:lib.rs-0353 */ /// True if `c` is considered horizontal whitespace according to Rust language definition.
/* FP:lib.rs-0354 */ pub fn is_horizontal_whitespace(c: char) -> bool {
/* FP:lib.rs-0355 */     // This is Pattern_White_Space.
/* FP:lib.rs-0356 */     //
/* FP:lib.rs-0357 */     // Note that this set is stable (ie, it doesn't change with different
/* FP:lib.rs-0358 */     // Unicode versions), so it's ok to just hard-code the values.
/* FP:lib.rs-0359 */ 
/* FP:lib.rs-0360 */     matches!(
/* FP:lib.rs-0361 */         c,
/* FP:lib.rs-0362 */         // Horizontal space characters
/* FP:lib.rs-0363 */         '\u{0009}'   // tab (\t)
/* FP:lib.rs-0364 */         | '\u{0020}' // space
/* FP:lib.rs-0365 */     )
/* FP:lib.rs-0366 */ }
/* FP:lib.rs-0367 */ 
/* FP:lib.rs-0368 */ /// True if `c` is valid as a first character of an identifier.
/* FP:lib.rs-0369 */ /// See [Rust language reference](https://doc.rust-lang.org/reference/identifiers.html) for
/* FP:lib.rs-0370 */ /// a formal definition of valid identifier name.
/* FP:lib.rs-0371 */ pub fn is_id_start(c: char) -> bool {
/* FP:lib.rs-0372 */     // This is XID_Start OR '_' (which formally is not a XID_Start).
/* FP:lib.rs-0373 */     c == '_' || unicode_xid::UnicodeXID::is_xid_start(c)
/* FP:lib.rs-0374 */ }
/* FP:lib.rs-0375 */ 
/* FP:lib.rs-0376 */ /// True if `c` is valid as a non-first character of an identifier.
/* FP:lib.rs-0377 */ /// See [Rust language reference](https://doc.rust-lang.org/reference/identifiers.html) for
/* FP:lib.rs-0378 */ /// a formal definition of valid identifier name.
/* FP:lib.rs-0379 */ pub fn is_id_continue(c: char) -> bool {
/* FP:lib.rs-0380 */     unicode_xid::UnicodeXID::is_xid_continue(c)
/* FP:lib.rs-0381 */ }
/* FP:lib.rs-0382 */ 
/* FP:lib.rs-0383 */ /// The passed string is lexically an identifier.
/* FP:lib.rs-0384 */ pub fn is_ident(string: &str) -> bool {
/* FP:lib.rs-0385 */     let mut chars = string.chars();
/* FP:lib.rs-0386 */     if let Some(start) = chars.next() {
/* FP:lib.rs-0387 */         is_id_start(start) && chars.all(is_id_continue)
/* FP:lib.rs-0388 */     } else {
/* FP:lib.rs-0389 */         false
/* FP:lib.rs-0390 */     }
/* FP:lib.rs-0391 */ }
/* FP:lib.rs-0392 */ 
/* FP:lib.rs-0393 */ impl Cursor<'_> {
/* FP:lib.rs-0394 */     /// Parses a token from the input string.
/* FP:lib.rs-0395 */     pub fn advance_token(&mut self) -> Token {
/* FP:lib.rs-0396 */         let Some(first_char) = self.bump() else {
/* FP:lib.rs-0397 */             return Token::new(TokenKind::Eof, 0);
/* FP:lib.rs-0398 */         };
/* FP:lib.rs-0399 */ 
/* FP:lib.rs-0400 */         let token_kind = match first_char {
/* FP:lib.rs-0401 */             c if matches!(self.frontmatter_allowed, FrontmatterAllowed::Yes)
/* FP:lib.rs-0402 */                 && is_whitespace(c) =>
/* FP:lib.rs-0403 */             {
/* FP:lib.rs-0404 */                 let mut last = first_char;
/* FP:lib.rs-0405 */                 while is_whitespace(self.first()) {
/* FP:lib.rs-0406 */                     let Some(c) = self.bump() else {
/* FP:lib.rs-0407 */                         break;
/* FP:lib.rs-0408 */                     };
/* FP:lib.rs-0409 */                     last = c;
/* FP:lib.rs-0410 */                 }
/* FP:lib.rs-0411 */                 // invalid frontmatter opening as whitespace preceding it isn't newline.
/* FP:lib.rs-0412 */                 // combine the whitespace and the frontmatter to a single token as we shall
/* FP:lib.rs-0413 */                 // error later.
/* FP:lib.rs-0414 */                 if last != '\n' && self.as_str().starts_with("---") {
/* FP:lib.rs-0415 */                     self.bump();
/* FP:lib.rs-0416 */                     self.frontmatter(true)
/* FP:lib.rs-0417 */                 } else {
/* FP:lib.rs-0418 */                     Whitespace
/* FP:lib.rs-0419 */                 }
/* FP:lib.rs-0420 */             }
/* FP:lib.rs-0421 */             '-' if matches!(self.frontmatter_allowed, FrontmatterAllowed::Yes)
/* FP:lib.rs-0422 */                 && self.as_str().starts_with("--") =>
/* FP:lib.rs-0423 */             {
/* FP:lib.rs-0424 */                 // happy path
/* FP:lib.rs-0425 */                 self.frontmatter(false)
/* FP:lib.rs-0426 */             }
/* FP:lib.rs-0427 */             // Slash, comment or block comment.
/* FP:lib.rs-0428 */             '/' => match self.first() {
/* FP:lib.rs-0429 */                 '/' => self.line_comment(),
/* FP:lib.rs-0430 */                 '*' => self.block_comment(),
/* FP:lib.rs-0431 */                 _ => Slash,
/* FP:lib.rs-0432 */             },
/* FP:lib.rs-0433 */ 
/* FP:lib.rs-0434 */             // Whitespace sequence.
/* FP:lib.rs-0435 */             c if is_whitespace(c) => self.whitespace(),
/* FP:lib.rs-0436 */ 
/* FP:lib.rs-0437 */             // Raw identifier, raw string literal or identifier.
/* FP:lib.rs-0438 */             'r' => match (self.first(), self.second()) {
/* FP:lib.rs-0439 */                 ('#', c1) if is_id_start(c1) => self.raw_ident(),
/* FP:lib.rs-0440 */                 ('#', _) | ('"', _) => {
/* FP:lib.rs-0441 */                     let res = self.raw_double_quoted_string(1);
/* FP:lib.rs-0442 */                     let suffix_start = self.pos_within_token();
/* FP:lib.rs-0443 */                     if res.is_ok() {
/* FP:lib.rs-0444 */                         self.eat_literal_suffix();
/* FP:lib.rs-0445 */                     }
/* FP:lib.rs-0446 */                     let kind = RawStr { n_hashes: res.ok() };
/* FP:lib.rs-0447 */                     Literal { kind, suffix_start }
/* FP:lib.rs-0448 */                 }
/* FP:lib.rs-0449 */                 _ => self.ident_or_unknown_prefix(),
/* FP:lib.rs-0450 */             },
/* FP:lib.rs-0451 */ 
/* FP:lib.rs-0452 */             // Byte literal, byte string literal, raw byte string literal or identifier.
/* FP:lib.rs-0453 */             'b' => self.c_or_byte_string(
/* FP:lib.rs-0454 */                 |terminated| ByteStr { terminated },
/* FP:lib.rs-0455 */                 |n_hashes| RawByteStr { n_hashes },
/* FP:lib.rs-0456 */                 Some(|terminated| Byte { terminated }),
/* FP:lib.rs-0457 */             ),
/* FP:lib.rs-0458 */ 
/* FP:lib.rs-0459 */             // c-string literal, raw c-string literal or identifier.
/* FP:lib.rs-0460 */             'c' => self.c_or_byte_string(
/* FP:lib.rs-0461 */                 |terminated| CStr { terminated },
/* FP:lib.rs-0462 */                 |n_hashes| RawCStr { n_hashes },
/* FP:lib.rs-0463 */                 None,
/* FP:lib.rs-0464 */             ),
/* FP:lib.rs-0465 */ 
/* FP:lib.rs-0466 */             // Identifier (this should be checked after other variant that can
/* FP:lib.rs-0467 */             // start as identifier).
/* FP:lib.rs-0468 */             c if is_id_start(c) => self.ident_or_unknown_prefix(),
/* FP:lib.rs-0469 */ 
/* FP:lib.rs-0470 */             // Numeric literal.
/* FP:lib.rs-0471 */             c @ '0'..='9' => {
/* FP:lib.rs-0472 */                 let literal_kind = self.number(c);
/* FP:lib.rs-0473 */                 let suffix_start = self.pos_within_token();
/* FP:lib.rs-0474 */                 self.eat_literal_suffix();
/* FP:lib.rs-0475 */                 TokenKind::Literal { kind: literal_kind, suffix_start }
/* FP:lib.rs-0476 */             }
/* FP:lib.rs-0477 */ 
/* FP:lib.rs-0478 */             // Guarded string literal prefix: `#"` or `##`
/* FP:lib.rs-0479 */             '#' if matches!(self.first(), '"' | '#') => {
/* FP:lib.rs-0480 */                 self.bump();
/* FP:lib.rs-0481 */                 TokenKind::GuardedStrPrefix
/* FP:lib.rs-0482 */             }
/* FP:lib.rs-0483 */ 
/* FP:lib.rs-0484 */             // One-symbol tokens.
/* FP:lib.rs-0485 */             ';' => Semi,
/* FP:lib.rs-0486 */             ',' => Comma,
/* FP:lib.rs-0487 */             '.' => Dot,
/* FP:lib.rs-0488 */             '(' => OpenParen,
/* FP:lib.rs-0489 */             ')' => CloseParen,
/* FP:lib.rs-0490 */             '{' => OpenBrace,
/* FP:lib.rs-0491 */             '}' => CloseBrace,
/* FP:lib.rs-0492 */             '[' => OpenBracket,
/* FP:lib.rs-0493 */             ']' => CloseBracket,
/* FP:lib.rs-0494 */             '@' => At,
/* FP:lib.rs-0495 */             '#' => Pound,
/* FP:lib.rs-0496 */             '~' => Tilde,
/* FP:lib.rs-0497 */             '?' => Question,
/* FP:lib.rs-0498 */             ':' => Colon,
/* FP:lib.rs-0499 */             '$' => Dollar,
/* FP:lib.rs-0500 */             '=' => Eq,
/* FP:lib.rs-0501 */             '!' => Bang,
/* FP:lib.rs-0502 */             '<' => Lt,
/* FP:lib.rs-0503 */             '>' => Gt,
/* FP:lib.rs-0504 */             '-' => Minus,
/* FP:lib.rs-0505 */             '&' => And,
/* FP:lib.rs-0506 */             '|' => Or,
/* FP:lib.rs-0507 */             '+' => Plus,
/* FP:lib.rs-0508 */             '*' => Star,
/* FP:lib.rs-0509 */             '^' => Caret,
/* FP:lib.rs-0510 */             '%' => Percent,
/* FP:lib.rs-0511 */ 
/* FP:lib.rs-0512 */             // Lifetime or character literal.
/* FP:lib.rs-0513 */             '\'' => self.lifetime_or_char(),
/* FP:lib.rs-0514 */ 
/* FP:lib.rs-0515 */             // String literal.
/* FP:lib.rs-0516 */             '"' => {
/* FP:lib.rs-0517 */                 let terminated = self.double_quoted_string();
/* FP:lib.rs-0518 */                 let suffix_start = self.pos_within_token();
/* FP:lib.rs-0519 */                 if terminated {
/* FP:lib.rs-0520 */                     self.eat_literal_suffix();
/* FP:lib.rs-0521 */                 }
/* FP:lib.rs-0522 */                 let kind = Str { terminated };
/* FP:lib.rs-0523 */                 Literal { kind, suffix_start }
/* FP:lib.rs-0524 */             }
/* FP:lib.rs-0525 */             // Identifier starting with an emoji. Only lexed for graceful error recovery.
/* FP:lib.rs-0526 */             c if !c.is_ascii() && c.is_emoji_char() => self.invalid_ident(),
/* FP:lib.rs-0527 */             _ => Unknown,
/* FP:lib.rs-0528 */         };
/* FP:lib.rs-0529 */         if matches!(self.frontmatter_allowed, FrontmatterAllowed::Yes)
/* FP:lib.rs-0530 */             && !matches!(token_kind, Whitespace)
/* FP:lib.rs-0531 */         {
/* FP:lib.rs-0532 */             // stop allowing frontmatters after first non-whitespace token
/* FP:lib.rs-0533 */             self.frontmatter_allowed = FrontmatterAllowed::No;
/* FP:lib.rs-0534 */         }
/* FP:lib.rs-0535 */         let res = Token::new(token_kind, self.pos_within_token());
/* FP:lib.rs-0536 */         self.reset_pos_within_token();
/* FP:lib.rs-0537 */         res
/* FP:lib.rs-0538 */     }
/* FP:lib.rs-0539 */ 
/* FP:lib.rs-0540 */     /// Given that one `-` was eaten, eat the rest of the frontmatter.
/* FP:lib.rs-0541 */     fn frontmatter(&mut self, has_invalid_preceding_whitespace: bool) -> TokenKind {
/* FP:lib.rs-0542 */         debug_assert_eq!('-', self.prev());
/* FP:lib.rs-0543 */ 
/* FP:lib.rs-0544 */         let pos = self.pos_within_token();
/* FP:lib.rs-0545 */         self.eat_while(|c| c == '-');
/* FP:lib.rs-0546 */ 
/* FP:lib.rs-0547 */         // one `-` is eaten by the caller.
/* FP:lib.rs-0548 */         let length_opening = self.pos_within_token() - pos + 1;
/* FP:lib.rs-0549 */ 
/* FP:lib.rs-0550 */         // must be ensured by the caller
/* FP:lib.rs-0551 */         debug_assert!(length_opening >= 3);
/* FP:lib.rs-0552 */ 
/* FP:lib.rs-0553 */         // whitespace between the opening and the infostring.
/* FP:lib.rs-0554 */         self.eat_while(|ch| ch != '\n' && is_horizontal_whitespace(ch));
/* FP:lib.rs-0555 */ 
/* FP:lib.rs-0556 */         // copied from `eat_identifier`, but allows `-` and `.` in infostring to allow something like
/* FP:lib.rs-0557 */         // `---Cargo.toml` as a valid opener
/* FP:lib.rs-0558 */         if is_id_start(self.first()) {
/* FP:lib.rs-0559 */             self.bump();
/* FP:lib.rs-0560 */             self.eat_while(|c| is_id_continue(c) || c == '-' || c == '.');
/* FP:lib.rs-0561 */         }
/* FP:lib.rs-0562 */ 
/* FP:lib.rs-0563 */         self.eat_while(|ch| ch != '\n' && is_horizontal_whitespace(ch));
/* FP:lib.rs-0564 */         let invalid_infostring = self.first() != '\n';
/* FP:lib.rs-0565 */ 
/* FP:lib.rs-0566 */         let mut found = false;
/* FP:lib.rs-0567 */         let nl_fence_pattern = format!("\n{:-<1$}", "", length_opening as usize);
/* FP:lib.rs-0568 */         if let Some(closing) = self.as_str().find(&nl_fence_pattern) {
/* FP:lib.rs-0569 */             // candidate found
/* FP:lib.rs-0570 */             self.bump_bytes(closing + nl_fence_pattern.len());
/* FP:lib.rs-0571 */             // in case like
/* FP:lib.rs-0572 */             // ---cargo
/* FP:lib.rs-0573 */             // --- blahblah
/* FP:lib.rs-0574 */             // or
/* FP:lib.rs-0575 */             // ---cargo
/* FP:lib.rs-0576 */             // ----
/* FP:lib.rs-0577 */             // combine those stuff into this frontmatter token such that it gets detected later.
/* FP:lib.rs-0578 */             self.eat_until(b'\n');
/* FP:lib.rs-0579 */             found = true;
/* FP:lib.rs-0580 */         }
/* FP:lib.rs-0581 */ 
/* FP:lib.rs-0582 */         if !found {
/* FP:lib.rs-0583 */             // recovery strategy: a closing statement might have preceding whitespace/newline
/* FP:lib.rs-0584 */             // but not have enough dashes to properly close. In this case, we eat until there,
/* FP:lib.rs-0585 */             // and report a mismatch in the parser.
/* FP:lib.rs-0586 */             let mut rest = self.as_str();
/* FP:lib.rs-0587 */             // We can look for a shorter closing (starting with four dashes but closing with three)
/* FP:lib.rs-0588 */             // and other indications that Rust has started and the infostring has ended.
/* FP:lib.rs-0589 */             let mut potential_closing = rest
/* FP:lib.rs-0590 */                 .find("\n---")
/* FP:lib.rs-0591 */                 // n.b. only in the case where there are dashes, we move the index to the line where
/* FP:lib.rs-0592 */                 // the dashes start as we eat to include that line. For other cases those are Rust code
/* FP:lib.rs-0593 */                 // and not included in the frontmatter.
/* FP:lib.rs-0594 */                 .map(|x| x + 1)
/* FP:lib.rs-0595 */                 .or_else(|| rest.find("\nuse "))
/* FP:lib.rs-0596 */                 .or_else(|| rest.find("\n//"))
/* FP:lib.rs-0597 */                 .or_else(|| rest.find("\n#["));
/* FP:lib.rs-0598 */ 
/* FP:lib.rs-0599 */             if potential_closing.is_none() {
/* FP:lib.rs-0600 */                 // a less fortunate recovery if all else fails which finds any dashes preceded by whitespace
/* FP:lib.rs-0601 */                 // on a standalone line. Might be wrong.
/* FP:lib.rs-0602 */                 while let Some(closing) = rest.find("---") {
/* FP:lib.rs-0603 */                     let preceding_chars_start = rest[..closing].rfind("\n").map_or(0, |i| i + 1);
/* FP:lib.rs-0604 */                     if rest[preceding_chars_start..closing].chars().all(is_horizontal_whitespace) {
/* FP:lib.rs-0605 */                         // candidate found
/* FP:lib.rs-0606 */                         potential_closing = Some(closing);
/* FP:lib.rs-0607 */                         break;
/* FP:lib.rs-0608 */                     } else {
/* FP:lib.rs-0609 */                         rest = &rest[closing + 3..];
/* FP:lib.rs-0610 */                     }
/* FP:lib.rs-0611 */                 }
/* FP:lib.rs-0612 */             }
/* FP:lib.rs-0613 */ 
/* FP:lib.rs-0614 */             if let Some(potential_closing) = potential_closing {
/* FP:lib.rs-0615 */                 // bump to the potential closing, and eat everything on that line.
/* FP:lib.rs-0616 */                 self.bump_bytes(potential_closing);
/* FP:lib.rs-0617 */                 self.eat_until(b'\n');
/* FP:lib.rs-0618 */             } else {
/* FP:lib.rs-0619 */                 // eat everything. this will get reported as an unclosed frontmatter.
/* FP:lib.rs-0620 */                 self.eat_while(|_| true);
/* FP:lib.rs-0621 */             }
/* FP:lib.rs-0622 */         }
/* FP:lib.rs-0623 */ 
/* FP:lib.rs-0624 */         Frontmatter { has_invalid_preceding_whitespace, invalid_infostring }
/* FP:lib.rs-0625 */     }
/* FP:lib.rs-0626 */ 
/* FP:lib.rs-0627 */     fn line_comment(&mut self) -> TokenKind {
/* FP:lib.rs-0628 */         debug_assert!(self.prev() == '/' && self.first() == '/');
/* FP:lib.rs-0629 */         self.bump();
/* FP:lib.rs-0630 */ 
/* FP:lib.rs-0631 */         let doc_style = match self.first() {
/* FP:lib.rs-0632 */             // `//` is an inner line doc comment.
/* FP:lib.rs-0633 */             '!' => Some(DocStyle::Inner),
/* FP:lib.rs-0634 */             // `////` (more than 3 slashes) is not considered a doc comment.
/* FP:lib.rs-0635 */             '/' if self.second() != '/' => Some(DocStyle::Outer),
/* FP:lib.rs-0636 */             _ => None,
/* FP:lib.rs-0637 */         };
/* FP:lib.rs-0638 */ 
/* FP:lib.rs-0639 */         self.eat_until(b'\n');
/* FP:lib.rs-0640 */         LineComment { doc_style }
/* FP:lib.rs-0641 */     }
/* FP:lib.rs-0642 */ 
/* FP:lib.rs-0643 */     fn block_comment(&mut self) -> TokenKind {
/* FP:lib.rs-0644 */         debug_assert!(self.prev() == '/' && self.first() == '*');
/* FP:lib.rs-0645 */         self.bump();
/* FP:lib.rs-0646 */ 
/* FP:lib.rs-0647 */         let doc_style = match self.first() {
/* FP:lib.rs-0648 */             // `/*` is an inner block doc comment.
/* FP:lib.rs-0649 */             '!' => Some(DocStyle::Inner),
/* FP:lib.rs-0650 */             // `/***` (more than 2 stars) is not considered a doc comment.
/* FP:lib.rs-0651 */             // `/**/` is not considered a doc comment.
/* FP:lib.rs-0652 */             '*' if !matches!(self.second(), '*' | '/') => Some(DocStyle::Outer),
/* FP:lib.rs-0653 */             _ => None,
/* FP:lib.rs-0654 */         };
/* FP:lib.rs-0655 */ 
/* FP:lib.rs-0656 */         let mut depth = 1usize;
/* FP:lib.rs-0657 */         while let Some(c) = self.bump() {
/* FP:lib.rs-0658 */             match c {
/* FP:lib.rs-0659 */                 '/' if self.first() == '*' => {
/* FP:lib.rs-0660 */                     self.bump();
/* FP:lib.rs-0661 */                     depth += 1;
/* FP:lib.rs-0662 */                 }
/* FP:lib.rs-0663 */                 '*' if self.first() == '/' => {
/* FP:lib.rs-0664 */                     self.bump();
/* FP:lib.rs-0665 */                     depth -= 1;
/* FP:lib.rs-0666 */                     if depth == 0 {
/* FP:lib.rs-0667 */                         // This block comment is closed, so for a construction like "/* */ */"
/* FP:lib.rs-0668 */                         // there will be a successfully parsed block comment "/* */"
/* FP:lib.rs-0669 */                         // and " */" will be processed separately.
/* FP:lib.rs-0670 */                         break;
/* FP:lib.rs-0671 */                     }
/* FP:lib.rs-0672 */                 }
/* FP:lib.rs-0673 */                 _ => (),
/* FP:lib.rs-0674 */             }
/* FP:lib.rs-0675 */         }
/* FP:lib.rs-0676 */ 
/* FP:lib.rs-0677 */         BlockComment { doc_style, terminated: depth == 0 }
/* FP:lib.rs-0678 */     }
/* FP:lib.rs-0679 */ 
/* FP:lib.rs-0680 */     fn whitespace(&mut self) -> TokenKind {
/* FP:lib.rs-0681 */         debug_assert!(is_whitespace(self.prev()));
/* FP:lib.rs-0682 */         self.eat_while(is_whitespace);
/* FP:lib.rs-0683 */         Whitespace
/* FP:lib.rs-0684 */     }
/* FP:lib.rs-0685 */ 
/* FP:lib.rs-0686 */     fn raw_ident(&mut self) -> TokenKind {
/* FP:lib.rs-0687 */         debug_assert!(self.prev() == 'r' && self.first() == '#' && is_id_start(self.second()));
/* FP:lib.rs-0688 */         // Eat "#" symbol.
/* FP:lib.rs-0689 */         self.bump();
/* FP:lib.rs-0690 */         // Eat the identifier part of RawIdent.
/* FP:lib.rs-0691 */         self.eat_identifier();
/* FP:lib.rs-0692 */         RawIdent
/* FP:lib.rs-0693 */     }
/* FP:lib.rs-0694 */ 
/* FP:lib.rs-0695 */     fn ident_or_unknown_prefix(&mut self) -> TokenKind {
/* FP:lib.rs-0696 */         debug_assert!(is_id_start(self.prev()));
/* FP:lib.rs-0697 */         // Start is already eaten, eat the rest of identifier.
/* FP:lib.rs-0698 */         self.eat_while(is_id_continue);
/* FP:lib.rs-0699 */         // Known prefixes must have been handled earlier. So if
/* FP:lib.rs-0700 */         // we see a prefix here, it is definitely an unknown prefix.
/* FP:lib.rs-0701 */         match self.first() {
/* FP:lib.rs-0702 */             '#' | '"' | '\'' => UnknownPrefix,
/* FP:lib.rs-0703 */             c if !c.is_ascii() && c.is_emoji_char() => self.invalid_ident(),
/* FP:lib.rs-0704 */             _ => Ident,
/* FP:lib.rs-0705 */         }
/* FP:lib.rs-0706 */     }
/* FP:lib.rs-0707 */ 
/* FP:lib.rs-0708 */     fn invalid_ident(&mut self) -> TokenKind {
/* FP:lib.rs-0709 */         // Start is already eaten, eat the rest of identifier.
/* FP:lib.rs-0710 */         self.eat_while(|c| {
/* FP:lib.rs-0711 */             const ZERO_WIDTH_JOINER: char = '\u{200d}';
/* FP:lib.rs-0712 */             is_id_continue(c) || (!c.is_ascii() && c.is_emoji_char()) || c == ZERO_WIDTH_JOINER
/* FP:lib.rs-0713 */         });
/* FP:lib.rs-0714 */         // An invalid identifier followed by '#' or '"' or '\'' could be
/* FP:lib.rs-0715 */         // interpreted as an invalid literal prefix. We don't bother doing that
/* FP:lib.rs-0716 */         // because the treatment of invalid identifiers and invalid prefixes
/* FP:lib.rs-0717 */         // would be the same.
/* FP:lib.rs-0718 */         InvalidIdent
/* FP:lib.rs-0719 */     }
/* FP:lib.rs-0720 */ 
/* FP:lib.rs-0721 */     fn c_or_byte_string(
/* FP:lib.rs-0722 */         &mut self,
/* FP:lib.rs-0723 */         mk_kind: fn(bool) -> LiteralKind,
/* FP:lib.rs-0724 */         mk_kind_raw: fn(Option<u8>) -> LiteralKind,
/* FP:lib.rs-0725 */         single_quoted: Option<fn(bool) -> LiteralKind>,
/* FP:lib.rs-0726 */     ) -> TokenKind {
/* FP:lib.rs-0727 */         match (self.first(), self.second(), single_quoted) {
/* FP:lib.rs-0728 */             ('\'', _, Some(single_quoted)) => {
/* FP:lib.rs-0729 */                 self.bump();
/* FP:lib.rs-0730 */                 let terminated = self.single_quoted_string();
/* FP:lib.rs-0731 */                 let suffix_start = self.pos_within_token();
/* FP:lib.rs-0732 */                 if terminated {
/* FP:lib.rs-0733 */                     self.eat_literal_suffix();
/* FP:lib.rs-0734 */                 }
/* FP:lib.rs-0735 */                 let kind = single_quoted(terminated);
/* FP:lib.rs-0736 */                 Literal { kind, suffix_start }
/* FP:lib.rs-0737 */             }
/* FP:lib.rs-0738 */             ('"', _, _) => {
/* FP:lib.rs-0739 */                 self.bump();
/* FP:lib.rs-0740 */                 let terminated = self.double_quoted_string();
/* FP:lib.rs-0741 */                 let suffix_start = self.pos_within_token();
/* FP:lib.rs-0742 */                 if terminated {
/* FP:lib.rs-0743 */                     self.eat_literal_suffix();
/* FP:lib.rs-0744 */                 }
/* FP:lib.rs-0745 */                 let kind = mk_kind(terminated);
/* FP:lib.rs-0746 */                 Literal { kind, suffix_start }
/* FP:lib.rs-0747 */             }
/* FP:lib.rs-0748 */             ('r', '"', _) | ('r', '#', _) => {
/* FP:lib.rs-0749 */                 self.bump();
/* FP:lib.rs-0750 */                 let res = self.raw_double_quoted_string(2);
/* FP:lib.rs-0751 */                 let suffix_start = self.pos_within_token();
/* FP:lib.rs-0752 */                 if res.is_ok() {
/* FP:lib.rs-0753 */                     self.eat_literal_suffix();
/* FP:lib.rs-0754 */                 }
/* FP:lib.rs-0755 */                 let kind = mk_kind_raw(res.ok());
/* FP:lib.rs-0756 */                 Literal { kind, suffix_start }
/* FP:lib.rs-0757 */             }
/* FP:lib.rs-0758 */             _ => self.ident_or_unknown_prefix(),
/* FP:lib.rs-0759 */         }
/* FP:lib.rs-0760 */     }
/* FP:lib.rs-0761 */ 
/* FP:lib.rs-0762 */     fn number(&mut self, first_digit: char) -> LiteralKind {
/* FP:lib.rs-0763 */         debug_assert!('0' <= self.prev() && self.prev() <= '9');
/* FP:lib.rs-0764 */         let mut base = Base::Decimal;
/* FP:lib.rs-0765 */         if first_digit == '0' {
/* FP:lib.rs-0766 */             // Attempt to parse encoding base.
/* FP:lib.rs-0767 */             match self.first() {
/* FP:lib.rs-0768 */                 'b' => {
/* FP:lib.rs-0769 */                     base = Base::Binary;
/* FP:lib.rs-0770 */                     self.bump();
/* FP:lib.rs-0771 */                     if !self.eat_decimal_digits() {
/* FP:lib.rs-0772 */                         return Int { base, empty_int: true };
/* FP:lib.rs-0773 */                     }
/* FP:lib.rs-0774 */                 }
/* FP:lib.rs-0775 */                 'o' => {
/* FP:lib.rs-0776 */                     base = Base::Octal;
/* FP:lib.rs-0777 */                     self.bump();
/* FP:lib.rs-0778 */                     if !self.eat_decimal_digits() {
/* FP:lib.rs-0779 */                         return Int { base, empty_int: true };
/* FP:lib.rs-0780 */                     }
/* FP:lib.rs-0781 */                 }
/* FP:lib.rs-0782 */                 'x' => {
/* FP:lib.rs-0783 */                     base = Base::Hexadecimal;
/* FP:lib.rs-0784 */                     self.bump();
/* FP:lib.rs-0785 */                     if !self.eat_hexadecimal_digits() {
/* FP:lib.rs-0786 */                         return Int { base, empty_int: true };
/* FP:lib.rs-0787 */                     }
/* FP:lib.rs-0788 */                 }
/* FP:lib.rs-0789 */                 // Not a base prefix; consume additional digits.
/* FP:lib.rs-0790 */                 '0'..='9' | '_' => {
/* FP:lib.rs-0791 */                     self.eat_decimal_digits();
/* FP:lib.rs-0792 */                 }
/* FP:lib.rs-0793 */ 
/* FP:lib.rs-0794 */                 // Also not a base prefix; nothing more to do here.
/* FP:lib.rs-0795 */                 '.' | 'e' | 'E' => {}
/* FP:lib.rs-0796 */ 
/* FP:lib.rs-0797 */                 // Just a 0.
/* FP:lib.rs-0798 */                 _ => return Int { base, empty_int: false },
/* FP:lib.rs-0799 */             }
/* FP:lib.rs-0800 */         } else {
/* FP:lib.rs-0801 */             // No base prefix, parse number in the usual way.
/* FP:lib.rs-0802 */             self.eat_decimal_digits();
/* FP:lib.rs-0803 */         }
/* FP:lib.rs-0804 */ 
/* FP:lib.rs-0805 */         match self.first() {
/* FP:lib.rs-0806 */             // Don't be greedy if this is actually an
/* FP:lib.rs-0807 */             // integer literal followed by field/method access or a range pattern
/* FP:lib.rs-0808 */             // (`0..2` and `12.foo()`)
/* FP:lib.rs-0809 */             '.' if self.second() != '.' && !is_id_start(self.second()) => {
/* FP:lib.rs-0810 */                 // might have stuff after the ., and if it does, it needs to start
/* FP:lib.rs-0811 */                 // with a number
/* FP:lib.rs-0812 */                 self.bump();
/* FP:lib.rs-0813 */                 let mut empty_exponent = false;
/* FP:lib.rs-0814 */                 if self.first().is_ascii_digit() {
/* FP:lib.rs-0815 */                     self.eat_decimal_digits();
/* FP:lib.rs-0816 */                     match self.first() {
/* FP:lib.rs-0817 */                         'e' | 'E' => {
/* FP:lib.rs-0818 */                             self.bump();
/* FP:lib.rs-0819 */                             empty_exponent = !self.eat_float_exponent();
/* FP:lib.rs-0820 */                         }
/* FP:lib.rs-0821 */                         _ => (),
/* FP:lib.rs-0822 */                     }
/* FP:lib.rs-0823 */                 }
/* FP:lib.rs-0824 */                 Float { base, empty_exponent }
/* FP:lib.rs-0825 */             }
/* FP:lib.rs-0826 */             'e' | 'E' => {
/* FP:lib.rs-0827 */                 self.bump();
/* FP:lib.rs-0828 */                 let empty_exponent = !self.eat_float_exponent();
/* FP:lib.rs-0829 */                 Float { base, empty_exponent }
/* FP:lib.rs-0830 */             }
/* FP:lib.rs-0831 */             _ => Int { base, empty_int: false },
/* FP:lib.rs-0832 */         }
/* FP:lib.rs-0833 */     }
/* FP:lib.rs-0834 */ 
/* FP:lib.rs-0835 */     fn lifetime_or_char(&mut self) -> TokenKind {
/* FP:lib.rs-0836 */         debug_assert!(self.prev() == '\'');
/* FP:lib.rs-0837 */ 
/* FP:lib.rs-0838 */         let can_be_a_lifetime = if self.second() == '\'' {
/* FP:lib.rs-0839 */             // It's surely not a lifetime.
/* FP:lib.rs-0840 */             false
/* FP:lib.rs-0841 */         } else {
/* FP:lib.rs-0842 */             // If the first symbol is valid for identifier, it can be a lifetime.
/* FP:lib.rs-0843 */             // Also check if it's a number for a better error reporting (so '0 will
/* FP:lib.rs-0844 */             // be reported as invalid lifetime and not as unterminated char literal).
/* FP:lib.rs-0845 */             is_id_start(self.first()) || self.first().is_ascii_digit()
/* FP:lib.rs-0846 */         };
/* FP:lib.rs-0847 */ 
/* FP:lib.rs-0848 */         if !can_be_a_lifetime {
/* FP:lib.rs-0849 */             let terminated = self.single_quoted_string();
/* FP:lib.rs-0850 */             let suffix_start = self.pos_within_token();
/* FP:lib.rs-0851 */             if terminated {
/* FP:lib.rs-0852 */                 self.eat_literal_suffix();
/* FP:lib.rs-0853 */             }
/* FP:lib.rs-0854 */             let kind = Char { terminated };
/* FP:lib.rs-0855 */             return Literal { kind, suffix_start };
/* FP:lib.rs-0856 */         }
/* FP:lib.rs-0857 */ 
/* FP:lib.rs-0858 */         if self.first() == 'r' && self.second() == '#' && is_id_start(self.third()) {
/* FP:lib.rs-0859 */             // Eat "r" and `#`, and identifier start characters.
/* FP:lib.rs-0860 */             self.bump();
/* FP:lib.rs-0861 */             self.bump();
/* FP:lib.rs-0862 */             self.bump();
/* FP:lib.rs-0863 */             self.eat_while(is_id_continue);
/* FP:lib.rs-0864 */             return RawLifetime;
/* FP:lib.rs-0865 */         }
/* FP:lib.rs-0866 */ 
/* FP:lib.rs-0867 */         // Either a lifetime or a character literal with
/* FP:lib.rs-0868 */         // length greater than 1.
/* FP:lib.rs-0869 */         let starts_with_number = self.first().is_ascii_digit();
/* FP:lib.rs-0870 */ 
/* FP:lib.rs-0871 */         // Skip the literal contents.
/* FP:lib.rs-0872 */         // First symbol can be a number (which isn't a valid identifier start),
/* FP:lib.rs-0873 */         // so skip it without any checks.
/* FP:lib.rs-0874 */         self.bump();
/* FP:lib.rs-0875 */         self.eat_while(is_id_continue);
/* FP:lib.rs-0876 */ 
/* FP:lib.rs-0877 */         match self.first() {
/* FP:lib.rs-0878 */             // Check if after skipping literal contents we've met a closing
/* FP:lib.rs-0879 */             // single quote (which means that user attempted to create a
/* FP:lib.rs-0880 */             // string with single quotes).
/* FP:lib.rs-0881 */             '\'' => {
/* FP:lib.rs-0882 */                 self.bump();
/* FP:lib.rs-0883 */                 let kind = Char { terminated: true };
/* FP:lib.rs-0884 */                 Literal { kind, suffix_start: self.pos_within_token() }
/* FP:lib.rs-0885 */             }
/* FP:lib.rs-0886 */             '#' if !starts_with_number => UnknownPrefixLifetime,
/* FP:lib.rs-0887 */             _ => Lifetime { starts_with_number },
/* FP:lib.rs-0888 */         }
/* FP:lib.rs-0889 */     }
/* FP:lib.rs-0890 */ 
/* FP:lib.rs-0891 */     fn single_quoted_string(&mut self) -> bool {
/* FP:lib.rs-0892 */         debug_assert!(self.prev() == '\'');
/* FP:lib.rs-0893 */         // Check if it's a one-symbol literal.
/* FP:lib.rs-0894 */         if self.second() == '\'' && self.first() != '\\' {
/* FP:lib.rs-0895 */             self.bump();
/* FP:lib.rs-0896 */             self.bump();
/* FP:lib.rs-0897 */             return true;
/* FP:lib.rs-0898 */         }
/* FP:lib.rs-0899 */ 
/* FP:lib.rs-0900 */         // Literal has more than one symbol.
/* FP:lib.rs-0901 */ 
/* FP:lib.rs-0902 */         // Parse until either quotes are terminated or error is detected.
/* FP:lib.rs-0903 */         loop {
/* FP:lib.rs-0904 */             match self.first() {
/* FP:lib.rs-0905 */                 // Quotes are terminated, finish parsing.
/* FP:lib.rs-0906 */                 '\'' => {
/* FP:lib.rs-0907 */                     self.bump();
/* FP:lib.rs-0908 */                     return true;
/* FP:lib.rs-0909 */                 }
/* FP:lib.rs-0910 */                 // Probably beginning of the comment, which we don't want to include
/* FP:lib.rs-0911 */                 // to the error report.
/* FP:lib.rs-0912 */                 '/' => break,
/* FP:lib.rs-0913 */                 // Newline without following '\'' means unclosed quote, stop parsing.
/* FP:lib.rs-0914 */                 '\n' if self.second() != '\'' => break,
/* FP:lib.rs-0915 */                 // End of file, stop parsing.
/* FP:lib.rs-0916 */                 EOF_CHAR if self.is_eof() => break,
/* FP:lib.rs-0917 */                 // Escaped slash is considered one character, so bump twice.
/* FP:lib.rs-0918 */                 '\\' => {
/* FP:lib.rs-0919 */                     self.bump();
/* FP:lib.rs-0920 */                     self.bump();
/* FP:lib.rs-0921 */                 }
/* FP:lib.rs-0922 */                 // Skip the character.
/* FP:lib.rs-0923 */                 _ => {
/* FP:lib.rs-0924 */                     self.bump();
/* FP:lib.rs-0925 */                 }
/* FP:lib.rs-0926 */             }
/* FP:lib.rs-0927 */         }
/* FP:lib.rs-0928 */         // String was not terminated.
/* FP:lib.rs-0929 */         false
/* FP:lib.rs-0930 */     }
/* FP:lib.rs-0931 */ 
/* FP:lib.rs-0932 */     /// Eats double-quoted string and returns true
/* FP:lib.rs-0933 */     /// if string is terminated.
/* FP:lib.rs-0934 */     fn double_quoted_string(&mut self) -> bool {
/* FP:lib.rs-0935 */         debug_assert!(self.prev() == '"');
/* FP:lib.rs-0936 */         while let Some(c) = self.bump() {
/* FP:lib.rs-0937 */             match c {
/* FP:lib.rs-0938 */                 '"' => {
/* FP:lib.rs-0939 */                     return true;
/* FP:lib.rs-0940 */                 }
/* FP:lib.rs-0941 */                 '\\' if self.first() == '\\' || self.first() == '"' => {
/* FP:lib.rs-0942 */                     // Bump again to skip escaped character.
/* FP:lib.rs-0943 */                     self.bump();
/* FP:lib.rs-0944 */                 }
/* FP:lib.rs-0945 */                 _ => (),
/* FP:lib.rs-0946 */             }
/* FP:lib.rs-0947 */         }
/* FP:lib.rs-0948 */         // End of file reached.
/* FP:lib.rs-0949 */         false
/* FP:lib.rs-0950 */     }
/* FP:lib.rs-0951 */ 
/* FP:lib.rs-0952 */     /// Attempt to lex for a guarded string literal.
/* FP:lib.rs-0953 */     ///
/* FP:lib.rs-0954 */     /// Used by `crate::rustc_parse::lexer` to lex for guarded strings
/* FP:lib.rs-0955 */     /// conditionally based on edition.
/* FP:lib.rs-0956 */     ///
/* FP:lib.rs-0957 */     /// Note: this will not reset the `Cursor` when a
/* FP:lib.rs-0958 */     /// guarded string is not found. It is the caller's
/* FP:lib.rs-0959 */     /// responsibility to do so.
/* FP:lib.rs-0960 */     pub fn guarded_double_quoted_string(&mut self) -> Option<GuardedStr> {
/* FP:lib.rs-0961 */         debug_assert!(self.prev() != '#');
/* FP:lib.rs-0962 */ 
/* FP:lib.rs-0963 */         let mut n_start_hashes: u32 = 0;
/* FP:lib.rs-0964 */         while self.first() == '#' {
/* FP:lib.rs-0965 */             n_start_hashes += 1;
/* FP:lib.rs-0966 */             self.bump();
/* FP:lib.rs-0967 */         }
/* FP:lib.rs-0968 */ 
/* FP:lib.rs-0969 */         if self.first() != '"' {
/* FP:lib.rs-0970 */             return None;
/* FP:lib.rs-0971 */         }
/* FP:lib.rs-0972 */         self.bump();
/* FP:lib.rs-0973 */         debug_assert!(self.prev() == '"');
/* FP:lib.rs-0974 */ 
/* FP:lib.rs-0975 */         // Lex the string itself as a normal string literal
/* FP:lib.rs-0976 */         // so we can recover that for older editions later.
/* FP:lib.rs-0977 */         let terminated = self.double_quoted_string();
/* FP:lib.rs-0978 */         if !terminated {
/* FP:lib.rs-0979 */             let token_len = self.pos_within_token();
/* FP:lib.rs-0980 */             self.reset_pos_within_token();
/* FP:lib.rs-0981 */ 
/* FP:lib.rs-0982 */             return Some(GuardedStr { n_hashes: n_start_hashes, terminated: false, token_len });
/* FP:lib.rs-0983 */         }
/* FP:lib.rs-0984 */ 
/* FP:lib.rs-0985 */         // Consume closing '#' symbols.
/* FP:lib.rs-0986 */         // Note that this will not consume extra trailing `#` characters:
/* FP:lib.rs-0987 */         // `###"abcde"####` is lexed as a `GuardedStr { n_end_hashes: 3, .. }`
/* FP:lib.rs-0988 */         // followed by a `#` token.
/* FP:lib.rs-0989 */         let mut n_end_hashes = 0;
/* FP:lib.rs-0990 */         while self.first() == '#' && n_end_hashes < n_start_hashes {
/* FP:lib.rs-0991 */             n_end_hashes += 1;
/* FP:lib.rs-0992 */             self.bump();
/* FP:lib.rs-0993 */         }
/* FP:lib.rs-0994 */ 
/* FP:lib.rs-0995 */         // Reserved syntax, always an error, so it doesn't matter if
/* FP:lib.rs-0996 */         // `n_start_hashes != n_end_hashes`.
/* FP:lib.rs-0997 */ 
/* FP:lib.rs-0998 */         self.eat_literal_suffix();
/* FP:lib.rs-0999 */ 
/* FP:lib.rs-1000 */         let token_len = self.pos_within_token();
/* FP:lib.rs-1001 */         self.reset_pos_within_token();
/* FP:lib.rs-1002 */ 
/* FP:lib.rs-1003 */         Some(GuardedStr { n_hashes: n_start_hashes, terminated: true, token_len })
/* FP:lib.rs-1004 */     }
/* FP:lib.rs-1005 */ 
/* FP:lib.rs-1006 */     /// Eats the double-quoted string and returns `n_hashes` and an error if encountered.
/* FP:lib.rs-1007 */     fn raw_double_quoted_string(&mut self, prefix_len: u32) -> Result<u8, RawStrError> {
/* FP:lib.rs-1008 */         // Wrap the actual function to handle the error with too many hashes.
/* FP:lib.rs-1009 */         // This way, it eats the whole raw string.
/* FP:lib.rs-1010 */         let n_hashes = self.raw_string_unvalidated(prefix_len)?;
/* FP:lib.rs-1011 */         // Only up to 255 `#`s are allowed in raw strings
/* FP:lib.rs-1012 */         match u8::try_from(n_hashes) {
/* FP:lib.rs-1013 */             Ok(num) => Ok(num),
/* FP:lib.rs-1014 */             Err(_) => Err(RawStrError::TooManyDelimiters { found: n_hashes }),
/* FP:lib.rs-1015 */         }
/* FP:lib.rs-1016 */     }
/* FP:lib.rs-1017 */ 
/* FP:lib.rs-1018 */     fn raw_string_unvalidated(&mut self, prefix_len: u32) -> Result<u32, RawStrError> {
/* FP:lib.rs-1019 */         debug_assert!(self.prev() == 'r');
/* FP:lib.rs-1020 */         let start_pos = self.pos_within_token();
/* FP:lib.rs-1021 */         let mut possible_terminator_offset = None;
/* FP:lib.rs-1022 */         let mut max_hashes = 0;
/* FP:lib.rs-1023 */ 
/* FP:lib.rs-1024 */         // Count opening '#' symbols.
/* FP:lib.rs-1025 */         let mut eaten = 0;
/* FP:lib.rs-1026 */         while self.first() == '#' {
/* FP:lib.rs-1027 */             eaten += 1;
/* FP:lib.rs-1028 */             self.bump();
/* FP:lib.rs-1029 */         }
/* FP:lib.rs-1030 */         let n_start_hashes = eaten;
/* FP:lib.rs-1031 */ 
/* FP:lib.rs-1032 */         // Check that string is started.
/* FP:lib.rs-1033 */         match self.bump() {
/* FP:lib.rs-1034 */             Some('"') => (),
/* FP:lib.rs-1035 */             c => {
/* FP:lib.rs-1036 */                 let c = c.unwrap_or(EOF_CHAR);
/* FP:lib.rs-1037 */                 return Err(RawStrError::InvalidStarter { bad_char: c });
/* FP:lib.rs-1038 */             }
/* FP:lib.rs-1039 */         }
/* FP:lib.rs-1040 */ 
/* FP:lib.rs-1041 */         // Skip the string contents and on each '#' character met, check if this is
/* FP:lib.rs-1042 */         // a raw string termination.
/* FP:lib.rs-1043 */         loop {
/* FP:lib.rs-1044 */             self.eat_until(b'"');
/* FP:lib.rs-1045 */ 
/* FP:lib.rs-1046 */             if self.is_eof() {
/* FP:lib.rs-1047 */                 return Err(RawStrError::NoTerminator {
/* FP:lib.rs-1048 */                     expected: n_start_hashes,
/* FP:lib.rs-1049 */                     found: max_hashes,
/* FP:lib.rs-1050 */                     possible_terminator_offset,
/* FP:lib.rs-1051 */                 });
/* FP:lib.rs-1052 */             }
/* FP:lib.rs-1053 */ 
/* FP:lib.rs-1054 */             // Eat closing double quote.
/* FP:lib.rs-1055 */             self.bump();
/* FP:lib.rs-1056 */ 
/* FP:lib.rs-1057 */             // Check that amount of closing '#' symbols
/* FP:lib.rs-1058 */             // is equal to the amount of opening ones.
/* FP:lib.rs-1059 */             // Note that this will not consume extra trailing `#` characters:
/* FP:lib.rs-1060 */             // `r###"abcde"####` is lexed as a `RawStr { n_hashes: 3 }`
/* FP:lib.rs-1061 */             // followed by a `#` token.
/* FP:lib.rs-1062 */             let mut n_end_hashes = 0;
/* FP:lib.rs-1063 */             while self.first() == '#' && n_end_hashes < n_start_hashes {
/* FP:lib.rs-1064 */                 n_end_hashes += 1;
/* FP:lib.rs-1065 */                 self.bump();
/* FP:lib.rs-1066 */             }
/* FP:lib.rs-1067 */ 
/* FP:lib.rs-1068 */             if n_end_hashes == n_start_hashes {
/* FP:lib.rs-1069 */                 return Ok(n_start_hashes);
/* FP:lib.rs-1070 */             } else if n_end_hashes > max_hashes {
/* FP:lib.rs-1071 */                 // Keep track of possible terminators to give a hint about
/* FP:lib.rs-1072 */                 // where there might be a missing terminator
/* FP:lib.rs-1073 */                 possible_terminator_offset =
/* FP:lib.rs-1074 */                     Some(self.pos_within_token() - start_pos - n_end_hashes + prefix_len);
/* FP:lib.rs-1075 */                 max_hashes = n_end_hashes;
/* FP:lib.rs-1076 */             }
/* FP:lib.rs-1077 */         }
/* FP:lib.rs-1078 */     }
/* FP:lib.rs-1079 */ 
/* FP:lib.rs-1080 */     fn eat_decimal_digits(&mut self) -> bool {
/* FP:lib.rs-1081 */         let mut has_digits = false;
/* FP:lib.rs-1082 */         loop {
/* FP:lib.rs-1083 */             match self.first() {
/* FP:lib.rs-1084 */                 '_' => {
/* FP:lib.rs-1085 */                     self.bump();
/* FP:lib.rs-1086 */                 }
/* FP:lib.rs-1087 */                 '0'..='9' => {
/* FP:lib.rs-1088 */                     has_digits = true;
/* FP:lib.rs-1089 */                     self.bump();
/* FP:lib.rs-1090 */                 }
/* FP:lib.rs-1091 */                 _ => break,
/* FP:lib.rs-1092 */             }
/* FP:lib.rs-1093 */         }
/* FP:lib.rs-1094 */         has_digits
/* FP:lib.rs-1095 */     }
/* FP:lib.rs-1096 */ 
/* FP:lib.rs-1097 */     fn eat_hexadecimal_digits(&mut self) -> bool {
/* FP:lib.rs-1098 */         let mut has_digits = false;
/* FP:lib.rs-1099 */         loop {
/* FP:lib.rs-1100 */             match self.first() {
/* FP:lib.rs-1101 */                 '_' => {
/* FP:lib.rs-1102 */                     self.bump();
/* FP:lib.rs-1103 */                 }
/* FP:lib.rs-1104 */                 '0'..='9' | 'a'..='f' | 'A'..='F' => {
/* FP:lib.rs-1105 */                     has_digits = true;
/* FP:lib.rs-1106 */                     self.bump();
/* FP:lib.rs-1107 */                 }
/* FP:lib.rs-1108 */                 _ => break,
/* FP:lib.rs-1109 */             }
/* FP:lib.rs-1110 */         }
/* FP:lib.rs-1111 */         has_digits
/* FP:lib.rs-1112 */     }
/* FP:lib.rs-1113 */ 
/* FP:lib.rs-1114 */     /// Eats the float exponent. Returns true if at least one digit was met,
/* FP:lib.rs-1115 */     /// and returns false otherwise.
/* FP:lib.rs-1116 */     fn eat_float_exponent(&mut self) -> bool {
/* FP:lib.rs-1117 */         debug_assert!(self.prev() == 'e' || self.prev() == 'E');
/* FP:lib.rs-1118 */         if self.first() == '-' || self.first() == '+' {
/* FP:lib.rs-1119 */             self.bump();
/* FP:lib.rs-1120 */         }
/* FP:lib.rs-1121 */         self.eat_decimal_digits()
/* FP:lib.rs-1122 */     }
/* FP:lib.rs-1123 */ 
/* FP:lib.rs-1124 */     // Eats the suffix of the literal, e.g. "u8".
/* FP:lib.rs-1125 */     fn eat_literal_suffix(&mut self) {
/* FP:lib.rs-1126 */         self.eat_identifier();
/* FP:lib.rs-1127 */     }
/* FP:lib.rs-1128 */ 
/* FP:lib.rs-1129 */     // Eats the identifier. Note: succeeds on `_`, which isn't a valid
/* FP:lib.rs-1130 */     // identifier.
/* FP:lib.rs-1131 */     fn eat_identifier(&mut self) {
/* FP:lib.rs-1132 */         if !is_id_start(self.first()) {
/* FP:lib.rs-1133 */             return;
/* FP:lib.rs-1134 */         }
/* FP:lib.rs-1135 */         self.bump();
/* FP:lib.rs-1136 */ 
/* FP:lib.rs-1137 */         self.eat_while(is_id_continue);
/* FP:lib.rs-1138 */     }
/* FP:lib.rs-1139 */ }