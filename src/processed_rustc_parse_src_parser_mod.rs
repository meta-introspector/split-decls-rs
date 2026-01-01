/* FP:mod.rs-0013 */ 
/* FP:mod.rs-0014 */ // Parsers for non-functionlike builtin macros are defined in rustc_parse so they can be used by
/* FP:mod.rs-0015 */ // both rustc_builtin_macros and rustfmt.
/* FP:mod.rs-0018 */ 
/* FP:mod.rs-0019 */ use std::assert_matches::debug_assert_matches;
/* FP:mod.rs-0020 */ use std::{fmt, mem, slice};
/* FP:mod.rs-0021 */ 
/* FP:mod.rs-0022 */ use attr_wrapper::{AttrWrapper, UsePreAttrPos};
/* FP:mod.rs-0023 */ pub use diagnostics::AttemptLocalParseRecovery;
/* FP:mod.rs-0024 */ pub(crate) use expr::ForbiddenLetReason;
/* FP:mod.rs-0025 */ // Public to use it for custom `if` expressions in rustfmt forks like https://github.com/tucant/rustfmt
/* FP:mod.rs-0026 */ pub use expr::LetChainsPolicy;
/* FP:mod.rs-0027 */ pub(crate) use item::{FnContext, FnParseMode};
/* FP:mod.rs-0028 */ pub use pat::{CommaRecoveryMode, RecoverColon, RecoverComma};
/* FP:mod.rs-0029 */ pub use path::PathStyle;
/* FP:mod.rs-0030 */ use crate::rustc_complete::token::{
/* FP:mod.rs-0031 */     self, IdentIsRaw, InvisibleOrigin, MetaVarKind, NtExprKind, NtPatKind, Token, TokenKind,
/* FP:mod.rs-0032 */ };
/* FP:mod.rs-0033 */ use crate::rustc_complete::tokenstream::{
/* FP:mod.rs-0034 */     ParserRange, ParserReplacement, Spacing, TokenCursor, TokenStream, TokenTree, TokenTreeCursor,
/* FP:mod.rs-0035 */ };
/* FP:mod.rs-0036 */ use crate::rustc_complete::util::case::Case;
/* FP:mod.rs-0037 */ use crate::rustc_complete::{
/* FP:mod.rs-0038 */     self as ast, AnonConst, AttrArgs, AttrId, ByRef, Const, CoroutineKind, DUMMY_NODE_ID,
/* FP:mod.rs-0039 */     DelimArgs, Expr, ExprKind, Extern, HasAttrs, HasTokens, Mutability, Recovered, Safety, StrLit,
/* FP:mod.rs-0040 */     Visibility, VisibilityKind,
/* FP:mod.rs-0041 */ };
/* FP:mod.rs-0042 */ use rustc_ast_pretty::pprust;
/* FP:mod.rs-0043 */ use crate::rustc_data_structures::fx::FxHashMap;
/* FP:mod.rs-0044 */ use crate::rustc_complete::{Applicability, Diag, FatalError, MultiSpan, PResult};
/* FP:mod.rs-0045 */ use crate::rustc_index::interval::IntervalSet;
/* FP:mod.rs-0046 */ use crate::rustc_complete::parse::ParseSess;
/* FP:mod.rs-0047 */ use crate::rustc_complete::{Ident, Span, Symbol, kw, sym};
/* FP:mod.rs-0048 */ use thin_vec::ThinVec;
/* FP:mod.rs-0049 */ use token_type::TokenTypeSet;
/* FP:mod.rs-0050 */ pub use token_type::{ExpKeywordPair, ExpTokenPair, TokenType};
/* FP:mod.rs-0051 */ use tracing::debug;
/* FP:mod.rs-0052 */ 
/* FP:mod.rs-0053 */ use crate::errors::{self, IncorrectVisibilityRestriction, NonStringAbiLiteral};
/* FP:mod.rs-0054 */ use crate::exp;
/* FP:mod.rs-0055 */ 
/* FP:mod.rs-0056 */ #[cfg(test)]
/* FP:mod.rs-0058 */ 
/* FP:mod.rs-0059 */ // Ideally, these tests would be in `rustc_ast`. But they depend on having a
/* FP:mod.rs-0060 */ // parser, so they are here.
/* FP:mod.rs-0061 */ #[cfg(test)]
/* FP:mod.rs-0062 */ mod tokenstream {
/* FP:mod.rs-0064 */ }
/* FP:mod.rs-0065 */ 
/* FP:mod.rs-0066 */ bitflags::bitflags! {
/* FP:mod.rs-0067 */     /// Restrictions applied while parsing.
/* FP:mod.rs-0068 */     ///
/* FP:mod.rs-0069 */     /// The parser maintains a bitset of restrictions it will honor while
/* FP:mod.rs-0070 */     /// parsing. This is essentially used as a way of tracking state of what
/* FP:mod.rs-0071 */     /// is being parsed and to change behavior based on that.
/* FP:mod.rs-0072 */     #[derive(Clone, Copy, Debug)]
/* FP:mod.rs-0073 */     struct Restrictions: u8 {
/* FP:mod.rs-0074 */         /// Restricts expressions for use in statement position.
/* FP:mod.rs-0075 */         ///
/* FP:mod.rs-0076 */         /// When expressions are used in various places, like statements or
/* FP:mod.rs-0077 */         /// match arms, this is used to stop parsing once certain tokens are
/* FP:mod.rs-0078 */         /// reached.
/* FP:mod.rs-0079 */         ///
/* FP:mod.rs-0080 */         /// For example, `if true {} & 1` with `STMT_EXPR` in effect is parsed
/* FP:mod.rs-0081 */         /// as two separate expression statements (`if` and a reference to 1).
/* FP:mod.rs-0082 */         /// Otherwise it is parsed as a bitwise AND where `if` is on the left
/* FP:mod.rs-0083 */         /// and 1 is on the right.
/* FP:mod.rs-0084 */         const STMT_EXPR         = 1 << 0;
/* FP:mod.rs-0085 */         /// Do not allow struct literals.
/* FP:mod.rs-0086 */         ///
/* FP:mod.rs-0087 */         /// There are several places in the grammar where we don't want to
/* FP:mod.rs-0088 */         /// allow struct literals because they can require lookahead, or
/* FP:mod.rs-0089 */         /// otherwise could be ambiguous or cause confusion. For example,
/* FP:mod.rs-0090 */         /// `if Foo {} {}` isn't clear if it is `Foo{}` struct literal, or
/* FP:mod.rs-0091 */         /// just `Foo` is the condition, followed by a consequent block,
/* FP:mod.rs-0092 */         /// followed by an empty block.
/* FP:mod.rs-0093 */         ///
/* FP:mod.rs-0094 */         /// See [RFC 92](https://rust-lang.github.io/rfcs/0092-struct-grammar.html).
/* FP:mod.rs-0095 */         const NO_STRUCT_LITERAL = 1 << 1;
/* FP:mod.rs-0096 */         /// Used to provide better error messages for const generic arguments.
/* FP:mod.rs-0097 */         ///
/* FP:mod.rs-0098 */         /// An un-braced const generic argument is limited to a very small
/* FP:mod.rs-0099 */         /// subset of expressions. This is used to detect the situation where
/* FP:mod.rs-0100 */         /// an expression outside of that subset is used, and to suggest to
/* FP:mod.rs-0101 */         /// wrap the expression in braces.
/* FP:mod.rs-0102 */         const CONST_EXPR        = 1 << 2;
/* FP:mod.rs-0103 */         /// Allows `let` expressions.
/* FP:mod.rs-0104 */         ///
/* FP:mod.rs-0105 */         /// `let pattern = scrutinee` is parsed as an expression, but it is
/* FP:mod.rs-0106 */         /// only allowed in let chains (`if` and `while` conditions).
/* FP:mod.rs-0107 */         /// Otherwise it is not an expression (note that `let` in statement
/* FP:mod.rs-0108 */         /// positions is treated as a `StmtKind::Let` statement, which has a
/* FP:mod.rs-0109 */         /// slightly different grammar).
/* FP:mod.rs-0110 */         const ALLOW_LET         = 1 << 3;
/* FP:mod.rs-0111 */         /// Used to detect a missing `=>` in a match guard.
/* FP:mod.rs-0112 */         ///
/* FP:mod.rs-0113 */         /// This is used for error handling in a match guard to give a better
/* FP:mod.rs-0114 */         /// error message if the `=>` is missing. It is set when parsing the
/* FP:mod.rs-0115 */         /// guard expression.
/* FP:mod.rs-0116 */         const IN_IF_GUARD       = 1 << 4;
/* FP:mod.rs-0117 */         /// Used to detect the incorrect use of expressions in patterns.
/* FP:mod.rs-0118 */         ///
/* FP:mod.rs-0119 */         /// This is used for error handling while parsing a pattern. During
/* FP:mod.rs-0120 */         /// error recovery, this will be set to try to parse the pattern as an
/* FP:mod.rs-0121 */         /// expression, but halts parsing the expression when reaching certain
/* FP:mod.rs-0122 */         /// tokens like `=`.
/* FP:mod.rs-0123 */         const IS_PAT            = 1 << 5;
/* FP:mod.rs-0124 */     }
/* FP:mod.rs-0125 */ }
/* FP:mod.rs-0126 */ 
/* FP:mod.rs-0127 */ #[derive(Clone, Copy, PartialEq, Debug)]
/* FP:mod.rs-0128 */ enum SemiColonMode {
/* FP:mod.rs-0129 */     Break,
/* FP:mod.rs-0130 */     Ignore,
/* FP:mod.rs-0131 */     Comma,
/* FP:mod.rs-0132 */ }
/* FP:mod.rs-0133 */ 
/* FP:mod.rs-0134 */ #[derive(Clone, Copy, PartialEq, Debug)]
/* FP:mod.rs-0135 */ enum BlockMode {
/* FP:mod.rs-0136 */     Break,
/* FP:mod.rs-0137 */     Ignore,
/* FP:mod.rs-0138 */ }
/* FP:mod.rs-0139 */ 
/* FP:mod.rs-0140 */ /// Whether or not we should force collection of tokens for an AST node,
/* FP:mod.rs-0141 */ /// regardless of whether or not it has attributes
/* FP:mod.rs-0142 */ #[derive(Clone, Copy, Debug, PartialEq)]
/* FP:mod.rs-0143 */ pub enum ForceCollect {
/* FP:mod.rs-0144 */     Yes,
/* FP:mod.rs-0145 */     No,
/* FP:mod.rs-0146 */ }
/* FP:mod.rs-0147 */ 
/* FP:mod.rs-0148 */ /// If the next tokens are ill-formed `$ty::` recover them as `<$ty>::`.
/* FP:mod.rs-0149 */ #[macro_export]
/* FP:mod.rs-0150 */ macro_rules! maybe_recover_from_interpolated_ty_qpath {
/* FP:mod.rs-0151 */     ($self: expr, $allow_qpath_recovery: expr) => {
/* FP:mod.rs-0152 */         if $allow_qpath_recovery
/* FP:mod.rs-0153 */             && $self.may_recover()
/* FP:mod.rs-0154 */             && let Some(mv_kind) = $self.token.is_metavar_seq()
/* FP:mod.rs-0155 */             && let token::MetaVarKind::Ty { .. } = mv_kind
/* FP:mod.rs-0156 */             && $self.check_noexpect_past_close_delim(&token::PathSep)
/* FP:mod.rs-0157 */         {
/* FP:mod.rs-0158 */             // Reparse the type, then move to recovery.
/* FP:mod.rs-0159 */             let ty = $self
/* FP:mod.rs-0160 */                 .eat_metavar_seq(mv_kind, |this| this.parse_ty_no_question_mark_recover())
/* FP:mod.rs-0161 */                 .expect("metavar seq ty");
/* FP:mod.rs-0162 */ 
/* FP:mod.rs-0163 */             return $self.maybe_recover_from_bad_qpath_stage_2($self.prev_token.span, ty);
/* FP:mod.rs-0164 */         }
/* FP:mod.rs-0165 */     };
/* FP:mod.rs-0166 */ }
/* FP:mod.rs-0167 */ 
/* FP:mod.rs-0168 */ #[derive(Clone, Copy, Debug)]
/* FP:mod.rs-0169 */ pub enum Recovery {
/* FP:mod.rs-0170 */     Allowed,
/* FP:mod.rs-0171 */     Forbidden,
/* FP:mod.rs-0172 */ }
/* FP:mod.rs-0173 */ 
/* FP:mod.rs-0174 */ #[derive(Clone)]
/* FP:mod.rs-0175 */ pub struct Parser<'a> {
/* FP:mod.rs-0176 */     pub psess: &'a ParseSess,
/* FP:mod.rs-0177 */     /// The current token.
/* FP:mod.rs-0178 */     pub token: Token,
/* FP:mod.rs-0179 */     /// The spacing for the current token.
/* FP:mod.rs-0180 */     token_spacing: Spacing,
/* FP:mod.rs-0181 */     /// The previous token.
/* FP:mod.rs-0182 */     pub prev_token: Token,
/* FP:mod.rs-0183 */     pub capture_cfg: bool,
/* FP:mod.rs-0184 */     restrictions: Restrictions,
/* FP:mod.rs-0185 */     expected_token_types: TokenTypeSet,
/* FP:mod.rs-0186 */     token_cursor: TokenCursor,
/* FP:mod.rs-0187 */     // The number of calls to `bump`, i.e. the position in the token stream.
/* FP:mod.rs-0188 */     num_bump_calls: u32,
/* FP:mod.rs-0189 */     // During parsing we may sometimes need to "unglue" a glued token into two
/* FP:mod.rs-0190 */     // or three component tokens (e.g. `>>` into `>` and `>`, or `>>=` into `>`
/* FP:mod.rs-0191 */     // and `>` and `=`), so the parser can consume them one at a time. This
/* FP:mod.rs-0192 */     // process bypasses the normal capturing mechanism (e.g. `num_bump_calls`
/* FP:mod.rs-0193 */     // will not be incremented), since the "unglued" tokens due not exist in
/* FP:mod.rs-0194 */     // the original `TokenStream`.
/* FP:mod.rs-0195 */     //
/* FP:mod.rs-0196 */     // If we end up consuming all the component tokens, this is not an issue,
/* FP:mod.rs-0197 */     // because we'll end up capturing the single "glued" token.
/* FP:mod.rs-0198 */     //
/* FP:mod.rs-0199 */     // However, sometimes we may want to capture not all of the original
/* FP:mod.rs-0200 */     // token. For example, capturing the `Vec<u8>` in `Option<Vec<u8>>`
/* FP:mod.rs-0201 */     // requires us to unglue the trailing `>>` token. The `break_last_token`
/* FP:mod.rs-0202 */     // field is used to track these tokens. They get appended to the captured
/* FP:mod.rs-0203 */     // stream when we evaluate a `LazyAttrTokenStream`.
/* FP:mod.rs-0204 */     //
/* FP:mod.rs-0205 */     // This value is always 0, 1, or 2. It can only reach 2 when splitting
/* FP:mod.rs-0206 */     // `>>=` or `<<=`.
/* FP:mod.rs-0207 */     break_last_token: u32,
/* FP:mod.rs-0208 */     /// This field is used to keep track of how many left angle brackets we have seen. This is
/* FP:mod.rs-0209 */     /// required in order to detect extra leading left angle brackets (`<` characters) and error
/* FP:mod.rs-0210 */     /// appropriately.
/* FP:mod.rs-0211 */     ///
/* FP:mod.rs-0212 */     /// See the comments in the `parse_path_segment` function for more details.
/* FP:mod.rs-0213 */     unmatched_angle_bracket_count: u16,
/* FP:mod.rs-0214 */     angle_bracket_nesting: u16,
/* FP:mod.rs-0215 */ 
/* FP:mod.rs-0216 */     last_unexpected_token_span: Option<Span>,
/* FP:mod.rs-0217 */     /// If present, this `Parser` is not parsing Rust code but rather a macro call.
/* FP:mod.rs-0218 */     subparser_name: Option<&'static str>,
/* FP:mod.rs-0219 */     capture_state: CaptureState,
/* FP:mod.rs-0220 */     /// This allows us to recover when the user forget to add braces around
/* FP:mod.rs-0221 */     /// multiple statements in the closure body.
/* FP:mod.rs-0222 */     current_closure: Option<ClosureSpans>,
/* FP:mod.rs-0223 */     /// Whether the parser is allowed to do recovery.
/* FP:mod.rs-0224 */     /// This is disabled when parsing macro arguments, see #103534
/* FP:mod.rs-0225 */     recovery: Recovery,
/* FP:mod.rs-0226 */ }
/* FP:mod.rs-0227 */ 
/* FP:mod.rs-0228 */ // This type is used a lot, e.g. it's cloned when matching many declarative macro rules with
/* FP:mod.rs-0229 */ // nonterminals. Make sure it doesn't unintentionally get bigger. We only check a few arches
/* FP:mod.rs-0230 */ // though, because `TokenTypeSet(u128)` alignment varies on others, changing the total size.
/* FP:mod.rs-0231 */ #[cfg(all(target_pointer_width = "64", any(target_arch = "aarch64", target_arch = "x86_64")))]
/* FP:mod.rs-0232 */ crate::rustc_data_structures::static_assert_size!(Parser<'_>, 288);
/* FP:mod.rs-0233 */ 
/* FP:mod.rs-0234 */ /// Stores span information about a closure.
/* FP:mod.rs-0235 */ #[derive(Clone, Debug)]
/* FP:mod.rs-0236 */ struct ClosureSpans {
/* FP:mod.rs-0237 */     whole_closure: Span,
/* FP:mod.rs-0238 */     closing_pipe: Span,
/* FP:mod.rs-0239 */     body: Span,
/* FP:mod.rs-0240 */ }
/* FP:mod.rs-0241 */ 
/* FP:mod.rs-0242 */ /// Controls how we capture tokens. Capturing can be expensive,
/* FP:mod.rs-0243 */ /// so we try to avoid performing capturing in cases where
/* FP:mod.rs-0244 */ /// we will never need an `AttrTokenStream`.
/* FP:mod.rs-0245 */ #[derive(Copy, Clone, Debug)]
/* FP:mod.rs-0246 */ enum Capturing {
/* FP:mod.rs-0247 */     /// We aren't performing any capturing - this is the default mode.
/* FP:mod.rs-0248 */     No,
/* FP:mod.rs-0249 */     /// We are capturing tokens
/* FP:mod.rs-0250 */     Yes,
/* FP:mod.rs-0251 */ }
/* FP:mod.rs-0252 */ 
/* FP:mod.rs-0253 */ // This state is used by `Parser::collect_tokens`.
/* FP:mod.rs-0254 */ #[derive(Clone, Debug)]
/* FP:mod.rs-0255 */ struct CaptureState {
/* FP:mod.rs-0256 */     capturing: Capturing,
/* FP:mod.rs-0257 */     parser_replacements: Vec<ParserReplacement>,
/* FP:mod.rs-0258 */     inner_attr_parser_ranges: FxHashMap<AttrId, ParserRange>,
/* FP:mod.rs-0259 */     // `IntervalSet` is good for perf because attrs are mostly added to this
/* FP:mod.rs-0260 */     // set in contiguous ranges.
/* FP:mod.rs-0261 */     seen_attrs: IntervalSet<AttrId>,
/* FP:mod.rs-0262 */ }
/* FP:mod.rs-0263 */ 
/* FP:mod.rs-0264 */ /// A sequence separator.
/* FP:mod.rs-0265 */ #[derive(Debug)]
/* FP:mod.rs-0266 */ struct SeqSep {
/* FP:mod.rs-0267 */     /// The separator token.
/* FP:mod.rs-0268 */     sep: Option<ExpTokenPair>,
/* FP:mod.rs-0269 */     /// `true` if a trailing separator is allowed.
/* FP:mod.rs-0270 */     trailing_sep_allowed: bool,
/* FP:mod.rs-0271 */ }
/* FP:mod.rs-0272 */ 
/* FP:mod.rs-0273 */ impl SeqSep {
/* FP:mod.rs-0274 */     fn trailing_allowed(sep: ExpTokenPair) -> SeqSep {
/* FP:mod.rs-0275 */         SeqSep { sep: Some(sep), trailing_sep_allowed: true }
/* FP:mod.rs-0276 */     }
/* FP:mod.rs-0277 */ 
/* FP:mod.rs-0278 */     fn none() -> SeqSep {
/* FP:mod.rs-0279 */         SeqSep { sep: None, trailing_sep_allowed: false }
/* FP:mod.rs-0280 */     }
/* FP:mod.rs-0281 */ }
/* FP:mod.rs-0282 */ 
/* FP:mod.rs-0283 */ #[derive(Debug)]
/* FP:mod.rs-0284 */ pub enum FollowedByType {
/* FP:mod.rs-0285 */     Yes,
/* FP:mod.rs-0286 */     No,
/* FP:mod.rs-0287 */ }
/* FP:mod.rs-0288 */ 
/* FP:mod.rs-0289 */ #[derive(Copy, Clone, Debug)]
/* FP:mod.rs-0290 */ pub enum Trailing {
/* FP:mod.rs-0291 */     No,
/* FP:mod.rs-0292 */     Yes,
/* FP:mod.rs-0293 */ }
/* FP:mod.rs-0294 */ 
/* FP:mod.rs-0295 */ impl From<bool> for Trailing {
/* FP:mod.rs-0296 */     fn from(b: bool) -> Trailing {
/* FP:mod.rs-0297 */         if b { Trailing::Yes } else { Trailing::No }
/* FP:mod.rs-0298 */     }
/* FP:mod.rs-0299 */ }
/* FP:mod.rs-0300 */ 
/* FP:mod.rs-0301 */ #[derive(Clone, Copy, Debug, PartialEq, Eq)]
/* FP:mod.rs-0302 */ pub(super) enum TokenDescription {
/* FP:mod.rs-0303 */     ReservedIdentifier,
/* FP:mod.rs-0304 */     Keyword,
/* FP:mod.rs-0305 */     ReservedKeyword,
/* FP:mod.rs-0306 */     DocComment,
/* FP:mod.rs-0307 */ 
/* FP:mod.rs-0308 */     // Expanded metavariables are wrapped in invisible delimiters which aren't
/* FP:mod.rs-0309 */     // pretty-printed. In error messages we must handle these specially
/* FP:mod.rs-0310 */     // otherwise we get confusing things in messages like "expected `(`, found
/* FP:mod.rs-0311 */     // ``". It's better to say e.g. "expected `(`, found type metavariable".
/* FP:mod.rs-0312 */     MetaVar(MetaVarKind),
/* FP:mod.rs-0313 */ }
/* FP:mod.rs-0314 */ 
/* FP:mod.rs-0315 */ impl TokenDescription {
/* FP:mod.rs-0316 */     pub(super) fn from_token(token: &Token) -> Option<Self> {
/* FP:mod.rs-0317 */         match token.kind {
/* FP:mod.rs-0318 */             _ if token.is_special_ident() => Some(TokenDescription::ReservedIdentifier),
/* FP:mod.rs-0319 */             _ if token.is_used_keyword() => Some(TokenDescription::Keyword),
/* FP:mod.rs-0320 */             _ if token.is_unused_keyword() => Some(TokenDescription::ReservedKeyword),
/* FP:mod.rs-0321 */             token::DocComment(..) => Some(TokenDescription::DocComment),
/* FP:mod.rs-0322 */             token::OpenInvisible(InvisibleOrigin::MetaVar(kind)) => {
/* FP:mod.rs-0323 */                 Some(TokenDescription::MetaVar(kind))
/* FP:mod.rs-0324 */             }
/* FP:mod.rs-0325 */             _ => None,
/* FP:mod.rs-0326 */         }
/* FP:mod.rs-0327 */     }
/* FP:mod.rs-0328 */ }
/* FP:mod.rs-0329 */ 
/* FP:mod.rs-0330 */ pub fn token_descr(token: &Token) -> String {
/* FP:mod.rs-0331 */     let s = pprust::token_to_string(token).to_string();
/* FP:mod.rs-0332 */ 
/* FP:mod.rs-0333 */     match (TokenDescription::from_token(token), &token.kind) {
/* FP:mod.rs-0334 */         (Some(TokenDescription::ReservedIdentifier), _) => format!("reserved identifier `{s}`"),
/* FP:mod.rs-0335 */         (Some(TokenDescription::Keyword), _) => format!("keyword `{s}`"),
/* FP:mod.rs-0336 */         (Some(TokenDescription::ReservedKeyword), _) => format!("reserved keyword `{s}`"),
/* FP:mod.rs-0337 */         (Some(TokenDescription::DocComment), _) => format!("doc comment `{s}`"),
/* FP:mod.rs-0338 */         // Deliberately doesn't print `s`, which is empty.
/* FP:mod.rs-0339 */         (Some(TokenDescription::MetaVar(kind)), _) => format!("`{kind}` metavariable"),
/* FP:mod.rs-0340 */         (None, TokenKind::NtIdent(..)) => format!("identifier `{s}`"),
/* FP:mod.rs-0341 */         (None, TokenKind::NtLifetime(..)) => format!("lifetime `{s}`"),
/* FP:mod.rs-0342 */         (None, _) => format!("`{s}`"),
/* FP:mod.rs-0343 */     }
/* FP:mod.rs-0344 */ }
/* FP:mod.rs-0345 */ 
/* FP:mod.rs-0346 */ impl<'a> Parser<'a> {
/* FP:mod.rs-0347 */     pub fn new(
/* FP:mod.rs-0348 */         psess: &'a ParseSess,
/* FP:mod.rs-0349 */         stream: TokenStream,
/* FP:mod.rs-0350 */         subparser_name: Option<&'static str>,
/* FP:mod.rs-0351 */     ) -> Self {
/* FP:mod.rs-0352 */         let mut parser = Parser {
/* FP:mod.rs-0353 */             psess,
/* FP:mod.rs-0354 */             token: Token::dummy(),
/* FP:mod.rs-0355 */             token_spacing: Spacing::Alone,
/* FP:mod.rs-0356 */             prev_token: Token::dummy(),
/* FP:mod.rs-0357 */             capture_cfg: false,
/* FP:mod.rs-0358 */             restrictions: Restrictions::empty(),
/* FP:mod.rs-0359 */             expected_token_types: TokenTypeSet::new(),
/* FP:mod.rs-0360 */             token_cursor: TokenCursor { curr: TokenTreeCursor::new(stream), stack: Vec::new() },
/* FP:mod.rs-0361 */             num_bump_calls: 0,
/* FP:mod.rs-0362 */             break_last_token: 0,
/* FP:mod.rs-0363 */             unmatched_angle_bracket_count: 0,
/* FP:mod.rs-0364 */             angle_bracket_nesting: 0,
/* FP:mod.rs-0365 */             last_unexpected_token_span: None,
/* FP:mod.rs-0366 */             subparser_name,
/* FP:mod.rs-0367 */             capture_state: CaptureState {
/* FP:mod.rs-0368 */                 capturing: Capturing::No,
/* FP:mod.rs-0369 */                 parser_replacements: Vec::new(),
/* FP:mod.rs-0370 */                 inner_attr_parser_ranges: Default::default(),
/* FP:mod.rs-0371 */                 seen_attrs: IntervalSet::new(u32::MAX as usize),
/* FP:mod.rs-0372 */             },
/* FP:mod.rs-0373 */             current_closure: None,
/* FP:mod.rs-0374 */             recovery: Recovery::Allowed,
/* FP:mod.rs-0375 */         };
/* FP:mod.rs-0376 */ 
/* FP:mod.rs-0377 */         // Make parser point to the first token.
/* FP:mod.rs-0378 */         parser.bump();
/* FP:mod.rs-0379 */ 
/* FP:mod.rs-0380 */         // Change this from 1 back to 0 after the bump. This eases debugging of
/* FP:mod.rs-0381 */         // `Parser::collect_tokens` because 0-indexed token positions are nicer
/* FP:mod.rs-0382 */         // than 1-indexed token positions.
/* FP:mod.rs-0383 */         parser.num_bump_calls = 0;
/* FP:mod.rs-0384 */ 
/* FP:mod.rs-0385 */         parser
/* FP:mod.rs-0386 */     }
/* FP:mod.rs-0387 */ 
/* FP:mod.rs-0388 */     #[inline]
/* FP:mod.rs-0389 */     pub fn recovery(mut self, recovery: Recovery) -> Self {
/* FP:mod.rs-0390 */         self.recovery = recovery;
/* FP:mod.rs-0391 */         self
/* FP:mod.rs-0392 */     }
/* FP:mod.rs-0393 */ 
/* FP:mod.rs-0394 */     #[inline]
/* FP:mod.rs-0395 */     fn with_recovery<T>(&mut self, recovery: Recovery, f: impl FnOnce(&mut Self) -> T) -> T {
/* FP:mod.rs-0396 */         let old = mem::replace(&mut self.recovery, recovery);
/* FP:mod.rs-0397 */         let res = f(self);
/* FP:mod.rs-0398 */         self.recovery = old;
/* FP:mod.rs-0399 */         res
/* FP:mod.rs-0400 */     }
/* FP:mod.rs-0401 */ 
/* FP:mod.rs-0402 */     /// Whether the parser is allowed to recover from broken code.
/* FP:mod.rs-0403 */     ///
/* FP:mod.rs-0404 */     /// If this returns false, recovering broken code into valid code (especially if this recovery does lookahead)
/* FP:mod.rs-0405 */     /// is not allowed. All recovery done by the parser must be gated behind this check.
/* FP:mod.rs-0406 */     ///
/* FP:mod.rs-0407 */     /// Technically, this only needs to restrict eager recovery by doing lookahead at more tokens.
/* FP:mod.rs-0408 */     /// But making the distinction is very subtle, and simply forbidding all recovery is a lot simpler to uphold.
/* FP:mod.rs-0409 */     #[inline]
/* FP:mod.rs-0410 */     fn may_recover(&self) -> bool {
/* FP:mod.rs-0411 */         matches!(self.recovery, Recovery::Allowed)
/* FP:mod.rs-0412 */     }
/* FP:mod.rs-0413 */ 
/* FP:mod.rs-0414 */     /// Version of [`unexpected`](Parser::unexpected) that "returns" any type in the `Ok`
/* FP:mod.rs-0415 */     /// (both those functions never return "Ok", and so can lie like that in the type).
/* FP:mod.rs-0416 */     pub fn unexpected_any<T>(&mut self) -> PResult<'a, T> {
/* FP:mod.rs-0417 */         match self.expect_one_of(&[], &[]) {
/* FP:mod.rs-0418 */             Err(e) => Err(e),
/* FP:mod.rs-0419 */             // We can get `Ok(true)` from `recover_closing_delimiter`
/* FP:mod.rs-0420 */             // which is called in `expected_one_of_not_found`.
/* FP:mod.rs-0421 */             Ok(_) => FatalError.raise(),
/* FP:mod.rs-0422 */         }
/* FP:mod.rs-0423 */     }
/* FP:mod.rs-0424 */ 
/* FP:mod.rs-0425 */     pub fn unexpected(&mut self) -> PResult<'a, ()> {
/* FP:mod.rs-0426 */         self.unexpected_any()
/* FP:mod.rs-0427 */     }
/* FP:mod.rs-0428 */ 
/* FP:mod.rs-0429 */     /// Expects and consumes the token `t`. Signals an error if the next token is not `t`.
/* FP:mod.rs-0430 */     pub fn expect(&mut self, exp: ExpTokenPair) -> PResult<'a, Recovered> {
/* FP:mod.rs-0431 */         if self.expected_token_types.is_empty() {
/* FP:mod.rs-0432 */             if self.token == exp.tok {
/* FP:mod.rs-0433 */                 self.bump();
/* FP:mod.rs-0434 */                 Ok(Recovered::No)
/* FP:mod.rs-0435 */             } else {
/* FP:mod.rs-0436 */                 self.unexpected_try_recover(&exp.tok)
/* FP:mod.rs-0437 */             }
/* FP:mod.rs-0438 */         } else {
/* FP:mod.rs-0439 */             self.expect_one_of(slice::from_ref(&exp), &[])
/* FP:mod.rs-0440 */         }
/* FP:mod.rs-0441 */     }
/* FP:mod.rs-0442 */ 
/* FP:mod.rs-0443 */     /// Expect next token to be edible or inedible token. If edible,
/* FP:mod.rs-0444 */     /// then consume it; if inedible, then return without consuming
/* FP:mod.rs-0445 */     /// anything. Signal a fatal error if next token is unexpected.
/* FP:mod.rs-0446 */     fn expect_one_of(
/* FP:mod.rs-0447 */         &mut self,
/* FP:mod.rs-0448 */         edible: &[ExpTokenPair],
/* FP:mod.rs-0449 */         inedible: &[ExpTokenPair],
/* FP:mod.rs-0450 */     ) -> PResult<'a, Recovered> {
/* FP:mod.rs-0451 */         if edible.iter().any(|exp| exp.tok == self.token.kind) {
/* FP:mod.rs-0452 */             self.bump();
/* FP:mod.rs-0453 */             Ok(Recovered::No)
/* FP:mod.rs-0454 */         } else if inedible.iter().any(|exp| exp.tok == self.token.kind) {
/* FP:mod.rs-0455 */             // leave it in the input
/* FP:mod.rs-0456 */             Ok(Recovered::No)
/* FP:mod.rs-0457 */         } else if self.token != token::Eof
/* FP:mod.rs-0458 */             && self.last_unexpected_token_span == Some(self.token.span)
/* FP:mod.rs-0459 */         {
/* FP:mod.rs-0460 */             FatalError.raise();
/* FP:mod.rs-0461 */         } else {
/* FP:mod.rs-0462 */             self.expected_one_of_not_found(edible, inedible)
/* FP:mod.rs-0463 */                 .map(|error_guaranteed| Recovered::Yes(error_guaranteed))
/* FP:mod.rs-0464 */         }
/* FP:mod.rs-0465 */     }
/* FP:mod.rs-0466 */ 
/* FP:mod.rs-0467 */     // Public for rustfmt usage.
/* FP:mod.rs-0468 */     pub fn parse_ident(&mut self) -> PResult<'a, Ident> {
/* FP:mod.rs-0469 */         self.parse_ident_common(true)
/* FP:mod.rs-0470 */     }
/* FP:mod.rs-0471 */ 
/* FP:mod.rs-0472 */     fn parse_ident_common(&mut self, recover: bool) -> PResult<'a, Ident> {
/* FP:mod.rs-0473 */         let (ident, is_raw) = self.ident_or_err(recover)?;
/* FP:mod.rs-0474 */ 
/* FP:mod.rs-0475 */         if matches!(is_raw, IdentIsRaw::No) && ident.is_reserved() {
/* FP:mod.rs-0476 */             let err = self.expected_ident_found_err();
/* FP:mod.rs-0477 */             if recover {
/* FP:mod.rs-0478 */                 err.emit();
/* FP:mod.rs-0479 */             } else {
/* FP:mod.rs-0480 */                 return Err(err);
/* FP:mod.rs-0481 */             }
/* FP:mod.rs-0482 */         }
/* FP:mod.rs-0483 */         self.bump();
/* FP:mod.rs-0484 */         Ok(ident)
/* FP:mod.rs-0485 */     }
/* FP:mod.rs-0486 */ 
/* FP:mod.rs-0487 */     fn ident_or_err(&mut self, recover: bool) -> PResult<'a, (Ident, IdentIsRaw)> {
/* FP:mod.rs-0488 */         match self.token.ident() {
/* FP:mod.rs-0489 */             Some(ident) => Ok(ident),
/* FP:mod.rs-0490 */             None => self.expected_ident_found(recover),
/* FP:mod.rs-0491 */         }
/* FP:mod.rs-0492 */     }
/* FP:mod.rs-0493 */ 
/* FP:mod.rs-0494 */     /// Checks if the next token is `tok`, and returns `true` if so.
/* FP:mod.rs-0495 */     ///
/* FP:mod.rs-0496 */     /// This method will automatically add `tok` to `expected_token_types` if `tok` is not
/* FP:mod.rs-0497 */     /// encountered.
/* FP:mod.rs-0498 */     #[inline]
/* FP:mod.rs-0499 */     pub fn check(&mut self, exp: ExpTokenPair) -> bool {
/* FP:mod.rs-0500 */         let is_present = self.token == exp.tok;
/* FP:mod.rs-0501 */         if !is_present {
/* FP:mod.rs-0502 */             self.expected_token_types.insert(exp.token_type);
/* FP:mod.rs-0503 */         }
/* FP:mod.rs-0504 */         is_present
/* FP:mod.rs-0505 */     }
/* FP:mod.rs-0506 */ 
/* FP:mod.rs-0507 */     #[inline]
/* FP:mod.rs-0508 */     #[must_use]
/* FP:mod.rs-0509 */     fn check_noexpect(&self, tok: &TokenKind) -> bool {
/* FP:mod.rs-0510 */         self.token == *tok
/* FP:mod.rs-0511 */     }
/* FP:mod.rs-0512 */ 
/* FP:mod.rs-0513 */     // Check the first token after the delimiter that closes the current
/* FP:mod.rs-0514 */     // delimited sequence. (Panics if used in the outermost token stream, which
/* FP:mod.rs-0515 */     // has no delimiters.) It uses a clone of the relevant tree cursor to skip
/* FP:mod.rs-0516 */     // past the entire `TokenTree::Delimited` in a single step, avoiding the
/* FP:mod.rs-0517 */     // need for unbounded token lookahead.
/* FP:mod.rs-0518 */     //
/* FP:mod.rs-0519 */     // Primarily used when `self.token` matches `OpenInvisible(_))`, to look
/* FP:mod.rs-0520 */     // ahead through the current metavar expansion.
/* FP:mod.rs-0521 */     fn check_noexpect_past_close_delim(&self, tok: &TokenKind) -> bool {
/* FP:mod.rs-0522 */         let mut tree_cursor = self.token_cursor.stack.last().unwrap().clone();
/* FP:mod.rs-0523 */         tree_cursor.bump();
/* FP:mod.rs-0524 */         matches!(
/* FP:mod.rs-0525 */             tree_cursor.curr(),
/* FP:mod.rs-0526 */             Some(TokenTree::Token(token::Token { kind, .. }, _)) if kind == tok
/* FP:mod.rs-0527 */         )
/* FP:mod.rs-0528 */     }
/* FP:mod.rs-0529 */ 
/* FP:mod.rs-0530 */     /// Consumes a token 'tok' if it exists. Returns whether the given token was present.
/* FP:mod.rs-0531 */     ///
/* FP:mod.rs-0532 */     /// the main purpose of this function is to reduce the cluttering of the suggestions list
/* FP:mod.rs-0533 */     /// which using the normal eat method could introduce in some cases.
/* FP:mod.rs-0534 */     #[inline]
/* FP:mod.rs-0535 */     #[must_use]
/* FP:mod.rs-0536 */     fn eat_noexpect(&mut self, tok: &TokenKind) -> bool {
/* FP:mod.rs-0537 */         let is_present = self.check_noexpect(tok);
/* FP:mod.rs-0538 */         if is_present {
/* FP:mod.rs-0539 */             self.bump()
/* FP:mod.rs-0540 */         }
/* FP:mod.rs-0541 */         is_present
/* FP:mod.rs-0542 */     }
/* FP:mod.rs-0543 */ 
/* FP:mod.rs-0544 */     /// Consumes a token 'tok' if it exists. Returns whether the given token was present.
/* FP:mod.rs-0545 */     #[inline]
/* FP:mod.rs-0546 */     #[must_use]
/* FP:mod.rs-0547 */     pub fn eat(&mut self, exp: ExpTokenPair) -> bool {
/* FP:mod.rs-0548 */         let is_present = self.check(exp);
/* FP:mod.rs-0549 */         if is_present {
/* FP:mod.rs-0550 */             self.bump()
/* FP:mod.rs-0551 */         }
/* FP:mod.rs-0552 */         is_present
/* FP:mod.rs-0553 */     }
/* FP:mod.rs-0554 */ 
/* FP:mod.rs-0555 */     /// If the next token is the given keyword, returns `true` without eating it.
/* FP:mod.rs-0556 */     /// An expectation is also added for diagnostics purposes.
/* FP:mod.rs-0557 */     #[inline]
/* FP:mod.rs-0558 */     #[must_use]
/* FP:mod.rs-0559 */     fn check_keyword(&mut self, exp: ExpKeywordPair) -> bool {
/* FP:mod.rs-0560 */         let is_keyword = self.token.is_keyword(exp.kw);
/* FP:mod.rs-0561 */         if !is_keyword {
/* FP:mod.rs-0562 */             self.expected_token_types.insert(exp.token_type);
/* FP:mod.rs-0563 */         }
/* FP:mod.rs-0564 */         is_keyword
/* FP:mod.rs-0565 */     }
/* FP:mod.rs-0566 */ 
/* FP:mod.rs-0567 */     #[inline]
/* FP:mod.rs-0568 */     #[must_use]
/* FP:mod.rs-0569 */     fn check_keyword_case(&mut self, exp: ExpKeywordPair, case: Case) -> bool {
/* FP:mod.rs-0570 */         if self.check_keyword(exp) {
/* FP:mod.rs-0571 */             true
/* FP:mod.rs-0572 */         } else if case == Case::Insensitive
/* FP:mod.rs-0573 */             && let Some((ident, IdentIsRaw::No)) = self.token.ident()
/* FP:mod.rs-0574 */             // Do an ASCII case-insensitive match, because all keywords are ASCII.
/* FP:mod.rs-0575 */             && ident.as_str().eq_ignore_ascii_case(exp.kw.as_str())
/* FP:mod.rs-0576 */         {
/* FP:mod.rs-0577 */             true
/* FP:mod.rs-0578 */         } else {
/* FP:mod.rs-0579 */             false
/* FP:mod.rs-0580 */         }
/* FP:mod.rs-0581 */     }
/* FP:mod.rs-0582 */ 
/* FP:mod.rs-0583 */     /// If the next token is the given keyword, eats it and returns `true`.
/* FP:mod.rs-0584 */     /// Otherwise, returns `false`. An expectation is also added for diagnostics purposes.
/* FP:mod.rs-0585 */     // Public for rustc_builtin_macros and rustfmt usage.
/* FP:mod.rs-0586 */     #[inline]
/* FP:mod.rs-0587 */     #[must_use]
/* FP:mod.rs-0588 */     pub fn eat_keyword(&mut self, exp: ExpKeywordPair) -> bool {
/* FP:mod.rs-0589 */         let is_keyword = self.check_keyword(exp);
/* FP:mod.rs-0590 */         if is_keyword {
/* FP:mod.rs-0591 */             self.bump();
/* FP:mod.rs-0592 */         }
/* FP:mod.rs-0593 */         is_keyword
/* FP:mod.rs-0594 */     }
/* FP:mod.rs-0595 */ 
/* FP:mod.rs-0596 */     /// Eats a keyword, optionally ignoring the case.
/* FP:mod.rs-0597 */     /// If the case differs (and is ignored) an error is issued.
/* FP:mod.rs-0598 */     /// This is useful for recovery.
/* FP:mod.rs-0599 */     #[inline]
/* FP:mod.rs-0600 */     #[must_use]
/* FP:mod.rs-0601 */     fn eat_keyword_case(&mut self, exp: ExpKeywordPair, case: Case) -> bool {
/* FP:mod.rs-0602 */         if self.eat_keyword(exp) {
/* FP:mod.rs-0603 */             true
/* FP:mod.rs-0604 */         } else if case == Case::Insensitive
/* FP:mod.rs-0605 */             && let Some((ident, IdentIsRaw::No)) = self.token.ident()
/* FP:mod.rs-0606 */             // Do an ASCII case-insensitive match, because all keywords are ASCII.
/* FP:mod.rs-0607 */             && ident.as_str().eq_ignore_ascii_case(exp.kw.as_str())
/* FP:mod.rs-0608 */         {
/* FP:mod.rs-0609 */             self.dcx().emit_err(errors::KwBadCase { span: ident.span, kw: exp.kw.as_str() });
/* FP:mod.rs-0610 */             self.bump();
/* FP:mod.rs-0611 */             true
/* FP:mod.rs-0612 */         } else {
/* FP:mod.rs-0613 */             false
/* FP:mod.rs-0614 */         }
/* FP:mod.rs-0615 */     }
/* FP:mod.rs-0616 */ 
/* FP:mod.rs-0617 */     /// If the next token is the given keyword, eats it and returns `true`.
/* FP:mod.rs-0618 */     /// Otherwise, returns `false`. No expectation is added.
/* FP:mod.rs-0619 */     // Public for rustc_builtin_macros usage.
/* FP:mod.rs-0620 */     #[inline]
/* FP:mod.rs-0621 */     #[must_use]
/* FP:mod.rs-0622 */     pub fn eat_keyword_noexpect(&mut self, kw: Symbol) -> bool {
/* FP:mod.rs-0623 */         let is_keyword = self.token.is_keyword(kw);
/* FP:mod.rs-0624 */         if is_keyword {
/* FP:mod.rs-0625 */             self.bump();
/* FP:mod.rs-0626 */         }
/* FP:mod.rs-0627 */         is_keyword
/* FP:mod.rs-0628 */     }
/* FP:mod.rs-0629 */ 
/* FP:mod.rs-0630 */     /// If the given word is not a keyword, signals an error.
/* FP:mod.rs-0631 */     /// If the next token is not the given word, signals an error.
/* FP:mod.rs-0632 */     /// Otherwise, eats it.
/* FP:mod.rs-0633 */     pub fn expect_keyword(&mut self, exp: ExpKeywordPair) -> PResult<'a, ()> {
/* FP:mod.rs-0634 */         if !self.eat_keyword(exp) { self.unexpected() } else { Ok(()) }
/* FP:mod.rs-0635 */     }
/* FP:mod.rs-0636 */ 
/* FP:mod.rs-0637 */     /// Consume a sequence produced by a metavar expansion, if present.
/* FP:mod.rs-0638 */     pub fn eat_metavar_seq<T>(
/* FP:mod.rs-0639 */         &mut self,
/* FP:mod.rs-0640 */         mv_kind: MetaVarKind,
/* FP:mod.rs-0641 */         f: impl FnMut(&mut Parser<'a>) -> PResult<'a, T>,
/* FP:mod.rs-0642 */     ) -> Option<T> {
/* FP:mod.rs-0643 */         self.eat_metavar_seq_with_matcher(|mvk| mvk == mv_kind, f)
/* FP:mod.rs-0644 */     }
/* FP:mod.rs-0645 */ 
/* FP:mod.rs-0646 */     /// A slightly more general form of `eat_metavar_seq`, for use with the
/* FP:mod.rs-0647 */     /// `MetaVarKind` variants that have parameters, where an exact match isn't
/* FP:mod.rs-0648 */     /// desired.
/* FP:mod.rs-0649 */     fn eat_metavar_seq_with_matcher<T>(
/* FP:mod.rs-0650 */         &mut self,
/* FP:mod.rs-0651 */         match_mv_kind: impl Fn(MetaVarKind) -> bool,
/* FP:mod.rs-0652 */         mut f: impl FnMut(&mut Parser<'a>) -> PResult<'a, T>,
/* FP:mod.rs-0653 */     ) -> Option<T> {
/* FP:mod.rs-0654 */         if let token::OpenInvisible(InvisibleOrigin::MetaVar(mv_kind)) = self.token.kind
/* FP:mod.rs-0655 */             && match_mv_kind(mv_kind)
/* FP:mod.rs-0656 */         {
/* FP:mod.rs-0657 */             self.bump();
/* FP:mod.rs-0658 */ 
/* FP:mod.rs-0659 */             // Recovery is disabled when parsing macro arguments, so it must
/* FP:mod.rs-0660 */             // also be disabled when reparsing pasted macro arguments,
/* FP:mod.rs-0661 */             // otherwise we get inconsistent results (e.g. #137874).
/* FP:mod.rs-0662 */             let res = self.with_recovery(Recovery::Forbidden, |this| f(this));
/* FP:mod.rs-0663 */ 
/* FP:mod.rs-0664 */             let res = match res {
/* FP:mod.rs-0665 */                 Ok(res) => res,
/* FP:mod.rs-0666 */                 Err(err) => {
/* FP:mod.rs-0667 */                     // This can occur in unusual error cases, e.g. #139445.
/* FP:mod.rs-0668 */                     err.delay_as_bug();
/* FP:mod.rs-0669 */                     return None;
/* FP:mod.rs-0670 */                 }
/* FP:mod.rs-0671 */             };
/* FP:mod.rs-0672 */ 
/* FP:mod.rs-0673 */             if let token::CloseInvisible(InvisibleOrigin::MetaVar(mv_kind)) = self.token.kind
/* FP:mod.rs-0674 */                 && match_mv_kind(mv_kind)
/* FP:mod.rs-0675 */             {
/* FP:mod.rs-0676 */                 self.bump();
/* FP:mod.rs-0677 */                 Some(res)
/* FP:mod.rs-0678 */             } else {
/* FP:mod.rs-0679 */                 // This can occur when invalid syntax is passed to a decl macro. E.g. see #139248,
/* FP:mod.rs-0680 */                 // where the reparse attempt of an invalid expr consumed the trailing invisible
/* FP:mod.rs-0681 */                 // delimiter.
/* FP:mod.rs-0682 */                 self.dcx()
/* FP:mod.rs-0683 */                     .span_delayed_bug(self.token.span, "no close delim with reparsing {mv_kind:?}");
/* FP:mod.rs-0684 */                 None
/* FP:mod.rs-0685 */             }
/* FP:mod.rs-0686 */         } else {
/* FP:mod.rs-0687 */             None
/* FP:mod.rs-0688 */         }
/* FP:mod.rs-0689 */     }
/* FP:mod.rs-0690 */ 
/* FP:mod.rs-0691 */     /// Is the given keyword `kw` followed by a non-reserved identifier?
/* FP:mod.rs-0692 */     fn is_kw_followed_by_ident(&self, kw: Symbol) -> bool {
/* FP:mod.rs-0693 */         self.token.is_keyword(kw) && self.look_ahead(1, |t| t.is_non_reserved_ident())
/* FP:mod.rs-0694 */     }
/* FP:mod.rs-0695 */ 
/* FP:mod.rs-0696 */     #[inline]
/* FP:mod.rs-0697 */     fn check_or_expected(&mut self, ok: bool, token_type: TokenType) -> bool {
/* FP:mod.rs-0698 */         if !ok {
/* FP:mod.rs-0699 */             self.expected_token_types.insert(token_type);
/* FP:mod.rs-0700 */         }
/* FP:mod.rs-0701 */         ok
/* FP:mod.rs-0702 */     }
/* FP:mod.rs-0703 */ 
/* FP:mod.rs-0704 */     fn check_ident(&mut self) -> bool {
/* FP:mod.rs-0705 */         self.check_or_expected(self.token.is_ident(), TokenType::Ident)
/* FP:mod.rs-0706 */     }
/* FP:mod.rs-0707 */ 
/* FP:mod.rs-0708 */     fn check_path(&mut self) -> bool {
/* FP:mod.rs-0709 */         self.check_or_expected(self.token.is_path_start(), TokenType::Path)
/* FP:mod.rs-0710 */     }
/* FP:mod.rs-0711 */ 
/* FP:mod.rs-0712 */     fn check_type(&mut self) -> bool {
/* FP:mod.rs-0713 */         self.check_or_expected(self.token.can_begin_type(), TokenType::Type)
/* FP:mod.rs-0714 */     }
/* FP:mod.rs-0715 */ 
/* FP:mod.rs-0716 */     fn check_const_arg(&mut self) -> bool {
/* FP:mod.rs-0717 */         self.check_or_expected(self.token.can_begin_const_arg(), TokenType::Const)
/* FP:mod.rs-0718 */     }
/* FP:mod.rs-0719 */ 
/* FP:mod.rs-0720 */     fn check_const_closure(&self) -> bool {
/* FP:mod.rs-0721 */         self.is_keyword_ahead(0, &[kw::Const])
/* FP:mod.rs-0722 */             && self.look_ahead(1, |t| match &t.kind {
/* FP:mod.rs-0723 */                 // async closures do not work with const closures, so we do not parse that here.
/* FP:mod.rs-0724 */                 token::Ident(kw::Move | kw::Use | kw::Static, IdentIsRaw::No)
/* FP:mod.rs-0725 */                 | token::OrOr
/* FP:mod.rs-0726 */                 | token::Or => true,
/* FP:mod.rs-0727 */                 _ => false,
/* FP:mod.rs-0728 */             })
/* FP:mod.rs-0729 */     }
/* FP:mod.rs-0730 */ 
/* FP:mod.rs-0731 */     fn check_inline_const(&self, dist: usize) -> bool {
/* FP:mod.rs-0732 */         self.is_keyword_ahead(dist, &[kw::Const])
/* FP:mod.rs-0733 */             && self.look_ahead(dist + 1, |t| match &t.kind {
/* FP:mod.rs-0734 */                 token::OpenBrace => true,
/* FP:mod.rs-0735 */                 token::OpenInvisible(InvisibleOrigin::MetaVar(MetaVarKind::Block)) => true,
/* FP:mod.rs-0736 */                 _ => false,
/* FP:mod.rs-0737 */             })
/* FP:mod.rs-0738 */     }
/* FP:mod.rs-0739 */ 
/* FP:mod.rs-0740 */     /// Checks to see if the next token is either `+` or `+=`.
/* FP:mod.rs-0741 */     /// Otherwise returns `false`.
/* FP:mod.rs-0742 */     #[inline]
/* FP:mod.rs-0743 */     fn check_plus(&mut self) -> bool {
/* FP:mod.rs-0744 */         self.check_or_expected(self.token.is_like_plus(), TokenType::Plus)
/* FP:mod.rs-0745 */     }
/* FP:mod.rs-0746 */ 
/* FP:mod.rs-0747 */     /// Eats the expected token if it's present possibly breaking
/* FP:mod.rs-0748 */     /// compound tokens like multi-character operators in process.
/* FP:mod.rs-0749 */     /// Returns `true` if the token was eaten.
/* FP:mod.rs-0750 */     fn break_and_eat(&mut self, exp: ExpTokenPair) -> bool {
/* FP:mod.rs-0751 */         if self.token == exp.tok {
/* FP:mod.rs-0752 */             self.bump();
/* FP:mod.rs-0753 */             return true;
/* FP:mod.rs-0754 */         }
/* FP:mod.rs-0755 */         match self.token.kind.break_two_token_op(1) {
/* FP:mod.rs-0756 */             Some((first, second)) if first == exp.tok => {
/* FP:mod.rs-0757 */                 let first_span = self.psess.source_map().start_point(self.token.span);
/* FP:mod.rs-0758 */                 let second_span = self.token.span.with_lo(first_span.hi());
/* FP:mod.rs-0759 */                 self.token = Token::new(first, first_span);
/* FP:mod.rs-0760 */                 // Keep track of this token - if we end token capturing now,
/* FP:mod.rs-0761 */                 // we'll want to append this token to the captured stream.
/* FP:mod.rs-0762 */                 //
/* FP:mod.rs-0763 */                 // If we consume any additional tokens, then this token
/* FP:mod.rs-0764 */                 // is not needed (we'll capture the entire 'glued' token),
/* FP:mod.rs-0765 */                 // and `bump` will set this field to 0.
/* FP:mod.rs-0766 */                 self.break_last_token += 1;
/* FP:mod.rs-0767 */                 // Use the spacing of the glued token as the spacing of the
/* FP:mod.rs-0768 */                 // unglued second token.
/* FP:mod.rs-0769 */                 self.bump_with((Token::new(second, second_span), self.token_spacing));
/* FP:mod.rs-0770 */                 true
/* FP:mod.rs-0771 */             }
/* FP:mod.rs-0772 */             _ => {
/* FP:mod.rs-0773 */                 self.expected_token_types.insert(exp.token_type);
/* FP:mod.rs-0774 */                 false
/* FP:mod.rs-0775 */             }
/* FP:mod.rs-0776 */         }
/* FP:mod.rs-0777 */     }
/* FP:mod.rs-0778 */ 
/* FP:mod.rs-0779 */     /// Eats `+` possibly breaking tokens like `+=` in process.
/* FP:mod.rs-0780 */     fn eat_plus(&mut self) -> bool {
/* FP:mod.rs-0781 */         self.break_and_eat(exp!(Plus))
/* FP:mod.rs-0782 */     }
/* FP:mod.rs-0783 */ 
/* FP:mod.rs-0784 */     /// Eats `&` possibly breaking tokens like `&&` in process.
/* FP:mod.rs-0785 */     /// Signals an error if `&` is not eaten.
/* FP:mod.rs-0786 */     fn expect_and(&mut self) -> PResult<'a, ()> {
/* FP:mod.rs-0787 */         if self.break_and_eat(exp!(And)) { Ok(()) } else { self.unexpected() }
/* FP:mod.rs-0788 */     }
/* FP:mod.rs-0789 */ 
/* FP:mod.rs-0790 */     /// Eats `|` possibly breaking tokens like `||` in process.
/* FP:mod.rs-0791 */     /// Signals an error if `|` was not eaten.
/* FP:mod.rs-0792 */     fn expect_or(&mut self) -> PResult<'a, ()> {
/* FP:mod.rs-0793 */         if self.break_and_eat(exp!(Or)) { Ok(()) } else { self.unexpected() }
/* FP:mod.rs-0794 */     }
/* FP:mod.rs-0795 */ 
/* FP:mod.rs-0796 */     /// Eats `<` possibly breaking tokens like `<<` in process.
/* FP:mod.rs-0797 */     fn eat_lt(&mut self) -> bool {
/* FP:mod.rs-0798 */         let ate = self.break_and_eat(exp!(Lt));
/* FP:mod.rs-0799 */         if ate {
/* FP:mod.rs-0800 */             // See doc comment for `unmatched_angle_bracket_count`.
/* FP:mod.rs-0801 */             self.unmatched_angle_bracket_count += 1;
/* FP:mod.rs-0802 */             debug!("eat_lt: (increment) count={:?}", self.unmatched_angle_bracket_count);
/* FP:mod.rs-0803 */         }
/* FP:mod.rs-0804 */         ate
/* FP:mod.rs-0805 */     }
/* FP:mod.rs-0806 */ 
/* FP:mod.rs-0807 */     /// Eats `<` possibly breaking tokens like `<<` in process.
/* FP:mod.rs-0808 */     /// Signals an error if `<` was not eaten.
/* FP:mod.rs-0809 */     fn expect_lt(&mut self) -> PResult<'a, ()> {
/* FP:mod.rs-0810 */         if self.eat_lt() { Ok(()) } else { self.unexpected() }
/* FP:mod.rs-0811 */     }
/* FP:mod.rs-0812 */ 
/* FP:mod.rs-0813 */     /// Eats `>` possibly breaking tokens like `>>` in process.
/* FP:mod.rs-0814 */     /// Signals an error if `>` was not eaten.
/* FP:mod.rs-0815 */     fn expect_gt(&mut self) -> PResult<'a, ()> {
/* FP:mod.rs-0816 */         if self.break_and_eat(exp!(Gt)) {
/* FP:mod.rs-0817 */             // See doc comment for `unmatched_angle_bracket_count`.
/* FP:mod.rs-0818 */             if self.unmatched_angle_bracket_count > 0 {
/* FP:mod.rs-0819 */                 self.unmatched_angle_bracket_count -= 1;
/* FP:mod.rs-0820 */                 debug!("expect_gt: (decrement) count={:?}", self.unmatched_angle_bracket_count);
/* FP:mod.rs-0821 */             }
/* FP:mod.rs-0822 */             Ok(())
/* FP:mod.rs-0823 */         } else {
/* FP:mod.rs-0824 */             self.unexpected()
/* FP:mod.rs-0825 */         }
/* FP:mod.rs-0826 */     }
/* FP:mod.rs-0827 */ 
/* FP:mod.rs-0828 */     /// Checks if the next token is contained within `closes`, and returns `true` if so.
/* FP:mod.rs-0829 */     fn expect_any_with_type(
/* FP:mod.rs-0830 */         &mut self,
/* FP:mod.rs-0831 */         closes_expected: &[ExpTokenPair],
/* FP:mod.rs-0832 */         closes_not_expected: &[&TokenKind],
/* FP:mod.rs-0833 */     ) -> bool {
/* FP:mod.rs-0834 */         closes_expected.iter().any(|&close| self.check(close))
/* FP:mod.rs-0835 */             || closes_not_expected.iter().any(|k| self.check_noexpect(k))
/* FP:mod.rs-0836 */     }
/* FP:mod.rs-0837 */ 
/* FP:mod.rs-0838 */     /// Parses a sequence until the specified delimiters. The function
/* FP:mod.rs-0839 */     /// `f` must consume tokens until reaching the next separator or
/* FP:mod.rs-0840 */     /// closing bracket.
/* FP:mod.rs-0841 */     fn parse_seq_to_before_tokens<T>(
/* FP:mod.rs-0842 */         &mut self,
/* FP:mod.rs-0843 */         closes_expected: &[ExpTokenPair],
/* FP:mod.rs-0844 */         closes_not_expected: &[&TokenKind],
/* FP:mod.rs-0845 */         sep: SeqSep,
/* FP:mod.rs-0846 */         mut f: impl FnMut(&mut Parser<'a>) -> PResult<'a, T>,
/* FP:mod.rs-0847 */     ) -> PResult<'a, (ThinVec<T>, Trailing, Recovered)> {
/* FP:mod.rs-0848 */         let mut first = true;
/* FP:mod.rs-0849 */         let mut recovered = Recovered::No;
/* FP:mod.rs-0850 */         let mut trailing = Trailing::No;
/* FP:mod.rs-0851 */         let mut v = ThinVec::new();
/* FP:mod.rs-0852 */ 
/* FP:mod.rs-0853 */         while !self.expect_any_with_type(closes_expected, closes_not_expected) {
/* FP:mod.rs-0854 */             if self.token.kind.is_close_delim_or_eof() {
/* FP:mod.rs-0855 */                 break;
/* FP:mod.rs-0856 */             }
/* FP:mod.rs-0857 */             if let Some(exp) = sep.sep {
/* FP:mod.rs-0858 */                 if first {
/* FP:mod.rs-0859 */                     // no separator for the first element
/* FP:mod.rs-0860 */                     first = false;
/* FP:mod.rs-0861 */                 } else {
/* FP:mod.rs-0862 */                     // check for separator
/* FP:mod.rs-0863 */                     match self.expect(exp) {
/* FP:mod.rs-0864 */                         Ok(Recovered::No) => {
/* FP:mod.rs-0865 */                             self.current_closure.take();
/* FP:mod.rs-0866 */                         }
/* FP:mod.rs-0867 */                         Ok(Recovered::Yes(guar)) => {
/* FP:mod.rs-0868 */                             self.current_closure.take();
/* FP:mod.rs-0869 */                             recovered = Recovered::Yes(guar);
/* FP:mod.rs-0870 */                             break;
/* FP:mod.rs-0871 */                         }
/* FP:mod.rs-0872 */                         Err(mut expect_err) => {
/* FP:mod.rs-0873 */                             let sp = self.prev_token.span.shrink_to_hi();
/* FP:mod.rs-0874 */                             let token_str = pprust::token_kind_to_string(&exp.tok);
/* FP:mod.rs-0875 */ 
/* FP:mod.rs-0876 */                             match self.current_closure.take() {
/* FP:mod.rs-0877 */                                 Some(closure_spans) if self.token == TokenKind::Semi => {
/* FP:mod.rs-0878 */                                     // Finding a semicolon instead of a comma
/* FP:mod.rs-0879 */                                     // after a closure body indicates that the
/* FP:mod.rs-0880 */                                     // closure body may be a block but the user
/* FP:mod.rs-0881 */                                     // forgot to put braces around its
/* FP:mod.rs-0882 */                                     // statements.
/* FP:mod.rs-0883 */ 
/* FP:mod.rs-0884 */                                     self.recover_missing_braces_around_closure_body(
/* FP:mod.rs-0885 */                                         closure_spans,
/* FP:mod.rs-0886 */                                         expect_err,
/* FP:mod.rs-0887 */                                     )?;
/* FP:mod.rs-0888 */ 
/* FP:mod.rs-0889 */                                     continue;
/* FP:mod.rs-0890 */                                 }
/* FP:mod.rs-0891 */ 
/* FP:mod.rs-0892 */                                 _ => {
/* FP:mod.rs-0893 */                                     // Attempt to keep parsing if it was a similar separator.
/* FP:mod.rs-0894 */                                     if exp.tok.similar_tokens().contains(&self.token.kind) {
/* FP:mod.rs-0895 */                                         self.bump();
/* FP:mod.rs-0896 */                                     }
/* FP:mod.rs-0897 */                                 }
/* FP:mod.rs-0898 */                             }
/* FP:mod.rs-0899 */ 
/* FP:mod.rs-0900 */                             // If this was a missing `@` in a binding pattern
/* FP:mod.rs-0901 */                             // bail with a suggestion
/* FP:mod.rs-0902 */                             // https://github.com/rust-lang/rust/issues/72373
/* FP:mod.rs-0903 */                             if self.prev_token.is_ident() && self.token == token::DotDot {
/* FP:mod.rs-0904 */                                 let msg = format!(
/* FP:mod.rs-0905 */                                     "if you meant to bind the contents of the rest of the array \
/* FP:mod.rs-0906 */                                      pattern into `{}`, use `@`",
/* FP:mod.rs-0907 */                                     pprust::token_to_string(&self.prev_token)
/* FP:mod.rs-0908 */                                 );
/* FP:mod.rs-0909 */                                 expect_err
/* FP:mod.rs-0910 */                                     .with_span_suggestion_verbose(
/* FP:mod.rs-0911 */                                         self.prev_token.span.shrink_to_hi().until(self.token.span),
/* FP:mod.rs-0912 */                                         msg,
/* FP:mod.rs-0913 */                                         " @ ",
/* FP:mod.rs-0914 */                                         Applicability::MaybeIncorrect,
/* FP:mod.rs-0915 */                                     )
/* FP:mod.rs-0916 */                                     .emit();
/* FP:mod.rs-0917 */                                 break;
/* FP:mod.rs-0918 */                             }
/* FP:mod.rs-0919 */ 
/* FP:mod.rs-0920 */                             // Attempt to keep parsing if it was an omitted separator.
/* FP:mod.rs-0921 */                             self.last_unexpected_token_span = None;
/* FP:mod.rs-0922 */                             match f(self) {
/* FP:mod.rs-0923 */                                 Ok(t) => {
/* FP:mod.rs-0924 */                                     // Parsed successfully, therefore most probably the code only
/* FP:mod.rs-0925 */                                     // misses a separator.
/* FP:mod.rs-0926 */                                     expect_err
/* FP:mod.rs-0927 */                                         .with_span_suggestion_short(
/* FP:mod.rs-0928 */                                             sp,
/* FP:mod.rs-0929 */                                             format!("missing `{token_str}`"),
/* FP:mod.rs-0930 */                                             token_str,
/* FP:mod.rs-0931 */                                             Applicability::MaybeIncorrect,
/* FP:mod.rs-0932 */                                         )
/* FP:mod.rs-0933 */                                         .emit();
/* FP:mod.rs-0934 */ 
/* FP:mod.rs-0935 */                                     v.push(t);
/* FP:mod.rs-0936 */                                     continue;
/* FP:mod.rs-0937 */                                 }
/* FP:mod.rs-0938 */                                 Err(e) => {
/* FP:mod.rs-0939 */                                     // Parsing failed, therefore it must be something more serious
/* FP:mod.rs-0940 */                                     // than just a missing separator.
/* FP:mod.rs-0941 */                                     for xx in &e.children {
/* FP:mod.rs-0942 */                                         // Propagate the help message from sub error `e` to main
/* FP:mod.rs-0943 */                                         // error `expect_err`.
/* FP:mod.rs-0944 */                                         expect_err.children.push(xx.clone());
/* FP:mod.rs-0945 */                                     }
/* FP:mod.rs-0946 */                                     e.cancel();
/* FP:mod.rs-0947 */                                     if self.token == token::Colon {
/* FP:mod.rs-0948 */                                         // We will try to recover in
/* FP:mod.rs-0949 */                                         // `maybe_recover_struct_lit_bad_delims`.
/* FP:mod.rs-0950 */                                         return Err(expect_err);
/* FP:mod.rs-0951 */                                     } else if let [exp] = closes_expected
/* FP:mod.rs-0952 */                                         && exp.token_type == TokenType::CloseParen
/* FP:mod.rs-0953 */                                     {
/* FP:mod.rs-0954 */                                         return Err(expect_err);
/* FP:mod.rs-0955 */                                     } else {
/* FP:mod.rs-0956 */                                         expect_err.emit();
/* FP:mod.rs-0957 */                                         break;
/* FP:mod.rs-0958 */                                     }
/* FP:mod.rs-0959 */                                 }
/* FP:mod.rs-0960 */                             }
/* FP:mod.rs-0961 */                         }
/* FP:mod.rs-0962 */                     }
/* FP:mod.rs-0963 */                 }
/* FP:mod.rs-0964 */             }
/* FP:mod.rs-0965 */             if sep.trailing_sep_allowed
/* FP:mod.rs-0966 */                 && self.expect_any_with_type(closes_expected, closes_not_expected)
/* FP:mod.rs-0967 */             {
/* FP:mod.rs-0968 */                 trailing = Trailing::Yes;
/* FP:mod.rs-0969 */                 break;
/* FP:mod.rs-0970 */             }
/* FP:mod.rs-0971 */ 
/* FP:mod.rs-0972 */             let t = f(self)?;
/* FP:mod.rs-0973 */             v.push(t);
/* FP:mod.rs-0974 */         }
/* FP:mod.rs-0975 */ 
/* FP:mod.rs-0976 */         Ok((v, trailing, recovered))
/* FP:mod.rs-0977 */     }
/* FP:mod.rs-0978 */ 
/* FP:mod.rs-0979 */     fn recover_missing_braces_around_closure_body(
/* FP:mod.rs-0980 */         &mut self,
/* FP:mod.rs-0981 */         closure_spans: ClosureSpans,
/* FP:mod.rs-0982 */         mut expect_err: Diag<'_>,
/* FP:mod.rs-0983 */     ) -> PResult<'a, ()> {
/* FP:mod.rs-0984 */         let initial_semicolon = self.token.span;
/* FP:mod.rs-0985 */ 
/* FP:mod.rs-0986 */         while self.eat(exp!(Semi)) {
/* FP:mod.rs-0987 */             let _ = self
/* FP:mod.rs-0988 */                 .parse_stmt_without_recovery(false, ForceCollect::No, false)
/* FP:mod.rs-0989 */                 .unwrap_or_else(|e| {
/* FP:mod.rs-0990 */                     e.cancel();
/* FP:mod.rs-0991 */                     None
/* FP:mod.rs-0992 */                 });
/* FP:mod.rs-0993 */         }
/* FP:mod.rs-0994 */ 
/* FP:mod.rs-0995 */         expect_err
/* FP:mod.rs-0996 */             .primary_message("closure bodies that contain statements must be surrounded by braces");
/* FP:mod.rs-0997 */ 
/* FP:mod.rs-0998 */         let preceding_pipe_span = closure_spans.closing_pipe;
/* FP:mod.rs-0999 */         let following_token_span = self.token.span;
/* FP:mod.rs-1000 */ 
/* FP:mod.rs-1001 */         let mut first_note = MultiSpan::from(vec![initial_semicolon]);
/* FP:mod.rs-1002 */         first_note.push_span_label(
/* FP:mod.rs-1003 */             initial_semicolon,
/* FP:mod.rs-1004 */             "this `;` turns the preceding closure into a statement",
/* FP:mod.rs-1005 */         );
/* FP:mod.rs-1006 */         first_note.push_span_label(
/* FP:mod.rs-1007 */             closure_spans.body,
/* FP:mod.rs-1008 */             "this expression is a statement because of the trailing semicolon",
/* FP:mod.rs-1009 */         );
/* FP:mod.rs-1010 */         expect_err.span_note(first_note, "statement found outside of a block");
/* FP:mod.rs-1011 */ 
/* FP:mod.rs-1012 */         let mut second_note = MultiSpan::from(vec![closure_spans.whole_closure]);
/* FP:mod.rs-1013 */         second_note.push_span_label(closure_spans.whole_closure, "this is the parsed closure...");
/* FP:mod.rs-1014 */         second_note.push_span_label(
/* FP:mod.rs-1015 */             following_token_span,
/* FP:mod.rs-1016 */             "...but likely you meant the closure to end here",
/* FP:mod.rs-1017 */         );
/* FP:mod.rs-1018 */         expect_err.span_note(second_note, "the closure body may be incorrectly delimited");
/* FP:mod.rs-1019 */ 
/* FP:mod.rs-1020 */         expect_err.span(vec![preceding_pipe_span, following_token_span]);
/* FP:mod.rs-1021 */ 
/* FP:mod.rs-1022 */         let opening_suggestion_str = " {".to_string();
/* FP:mod.rs-1023 */         let closing_suggestion_str = "}".to_string();
/* FP:mod.rs-1024 */ 
/* FP:mod.rs-1025 */         expect_err.multipart_suggestion(
/* FP:mod.rs-1026 */             "try adding braces",
/* FP:mod.rs-1027 */             vec![
/* FP:mod.rs-1028 */                 (preceding_pipe_span.shrink_to_hi(), opening_suggestion_str),
/* FP:mod.rs-1029 */                 (following_token_span.shrink_to_lo(), closing_suggestion_str),
/* FP:mod.rs-1030 */             ],
/* FP:mod.rs-1031 */             Applicability::MaybeIncorrect,
/* FP:mod.rs-1032 */         );
/* FP:mod.rs-1033 */ 
/* FP:mod.rs-1034 */         expect_err.emit();
/* FP:mod.rs-1035 */ 
/* FP:mod.rs-1036 */         Ok(())
/* FP:mod.rs-1037 */     }
/* FP:mod.rs-1038 */ 
/* FP:mod.rs-1039 */     /// Parses a sequence, not including the delimiters. The function
/* FP:mod.rs-1040 */     /// `f` must consume tokens until reaching the next separator or
/* FP:mod.rs-1041 */     /// closing bracket.
/* FP:mod.rs-1042 */     fn parse_seq_to_before_end<T>(
/* FP:mod.rs-1043 */         &mut self,
/* FP:mod.rs-1044 */         close: ExpTokenPair,
/* FP:mod.rs-1045 */         sep: SeqSep,
/* FP:mod.rs-1046 */         f: impl FnMut(&mut Parser<'a>) -> PResult<'a, T>,
/* FP:mod.rs-1047 */     ) -> PResult<'a, (ThinVec<T>, Trailing, Recovered)> {
/* FP:mod.rs-1048 */         self.parse_seq_to_before_tokens(&[close], &[], sep, f)
/* FP:mod.rs-1049 */     }
/* FP:mod.rs-1050 */ 
/* FP:mod.rs-1051 */     /// Parses a sequence, including only the closing delimiter. The function
/* FP:mod.rs-1052 */     /// `f` must consume tokens until reaching the next separator or
/* FP:mod.rs-1053 */     /// closing bracket.
/* FP:mod.rs-1054 */     fn parse_seq_to_end<T>(
/* FP:mod.rs-1055 */         &mut self,
/* FP:mod.rs-1056 */         close: ExpTokenPair,
/* FP:mod.rs-1057 */         sep: SeqSep,
/* FP:mod.rs-1058 */         f: impl FnMut(&mut Parser<'a>) -> PResult<'a, T>,
/* FP:mod.rs-1059 */     ) -> PResult<'a, (ThinVec<T>, Trailing)> {
/* FP:mod.rs-1060 */         let (val, trailing, recovered) = self.parse_seq_to_before_end(close, sep, f)?;
/* FP:mod.rs-1061 */         if matches!(recovered, Recovered::No) && !self.eat(close) {
/* FP:mod.rs-1062 */             self.dcx().span_delayed_bug(
/* FP:mod.rs-1063 */                 self.token.span,
/* FP:mod.rs-1064 */                 "recovered but `parse_seq_to_before_end` did not give us the close token",
/* FP:mod.rs-1065 */             );
/* FP:mod.rs-1066 */         }
/* FP:mod.rs-1067 */         Ok((val, trailing))
/* FP:mod.rs-1068 */     }
/* FP:mod.rs-1069 */ 
/* FP:mod.rs-1070 */     /// Parses a sequence, including both delimiters. The function
/* FP:mod.rs-1071 */     /// `f` must consume tokens until reaching the next separator or
/* FP:mod.rs-1072 */     /// closing bracket.
/* FP:mod.rs-1073 */     fn parse_unspanned_seq<T>(
/* FP:mod.rs-1074 */         &mut self,
/* FP:mod.rs-1075 */         open: ExpTokenPair,
/* FP:mod.rs-1076 */         close: ExpTokenPair,
/* FP:mod.rs-1077 */         sep: SeqSep,
/* FP:mod.rs-1078 */         f: impl FnMut(&mut Parser<'a>) -> PResult<'a, T>,
/* FP:mod.rs-1079 */     ) -> PResult<'a, (ThinVec<T>, Trailing)> {
/* FP:mod.rs-1080 */         self.expect(open)?;
/* FP:mod.rs-1081 */         self.parse_seq_to_end(close, sep, f)
/* FP:mod.rs-1082 */     }
/* FP:mod.rs-1083 */ 
/* FP:mod.rs-1084 */     /// Parses a comma-separated sequence, including both delimiters.
/* FP:mod.rs-1085 */     /// The function `f` must consume tokens until reaching the next separator or
/* FP:mod.rs-1086 */     /// closing bracket.
/* FP:mod.rs-1087 */     fn parse_delim_comma_seq<T>(
/* FP:mod.rs-1088 */         &mut self,
/* FP:mod.rs-1089 */         open: ExpTokenPair,
/* FP:mod.rs-1090 */         close: ExpTokenPair,
/* FP:mod.rs-1091 */         f: impl FnMut(&mut Parser<'a>) -> PResult<'a, T>,
/* FP:mod.rs-1092 */     ) -> PResult<'a, (ThinVec<T>, Trailing)> {
/* FP:mod.rs-1093 */         self.parse_unspanned_seq(open, close, SeqSep::trailing_allowed(exp!(Comma)), f)
/* FP:mod.rs-1094 */     }
/* FP:mod.rs-1095 */ 
/* FP:mod.rs-1096 */     /// Parses a comma-separated sequence delimited by parentheses (e.g. `(x, y)`).
/* FP:mod.rs-1097 */     /// The function `f` must consume tokens until reaching the next separator or
/* FP:mod.rs-1098 */     /// closing bracket.
/* FP:mod.rs-1099 */     pub fn parse_paren_comma_seq<T>(
/* FP:mod.rs-1100 */         &mut self,
/* FP:mod.rs-1101 */         f: impl FnMut(&mut Parser<'a>) -> PResult<'a, T>,
/* FP:mod.rs-1102 */     ) -> PResult<'a, (ThinVec<T>, Trailing)> {
/* FP:mod.rs-1103 */         self.parse_delim_comma_seq(exp!(OpenParen), exp!(CloseParen), f)
/* FP:mod.rs-1104 */     }
/* FP:mod.rs-1105 */ 
/* FP:mod.rs-1106 */     /// Advance the parser by one token using provided token as the next one.
/* FP:mod.rs-1107 */     fn bump_with(&mut self, next: (Token, Spacing)) {
/* FP:mod.rs-1108 */         self.inlined_bump_with(next)
/* FP:mod.rs-1109 */     }
/* FP:mod.rs-1110 */ 
/* FP:mod.rs-1111 */     /// This always-inlined version should only be used on hot code paths.
/* FP:mod.rs-1112 */     #[inline(always)]
/* FP:mod.rs-1113 */     fn inlined_bump_with(&mut self, (next_token, next_spacing): (Token, Spacing)) {
/* FP:mod.rs-1114 */         // Update the current and previous tokens.
/* FP:mod.rs-1115 */         self.prev_token = mem::replace(&mut self.token, next_token);
/* FP:mod.rs-1116 */         self.token_spacing = next_spacing;
/* FP:mod.rs-1117 */ 
/* FP:mod.rs-1118 */         // Diagnostics.
/* FP:mod.rs-1119 */         self.expected_token_types.clear();
/* FP:mod.rs-1120 */     }
/* FP:mod.rs-1121 */ 
/* FP:mod.rs-1122 */     /// Advance the parser by one token.
/* FP:mod.rs-1123 */     pub fn bump(&mut self) {
/* FP:mod.rs-1124 */         // Note: destructuring here would give nicer code, but it was found in #96210 to be slower
/* FP:mod.rs-1125 */         // than `.0`/`.1` access.
/* FP:mod.rs-1126 */         let mut next = self.token_cursor.inlined_next();
/* FP:mod.rs-1127 */         self.num_bump_calls += 1;
/* FP:mod.rs-1128 */         // We got a token from the underlying cursor and no longer need to
/* FP:mod.rs-1129 */         // worry about an unglued token. See `break_and_eat` for more details.
/* FP:mod.rs-1130 */         self.break_last_token = 0;
/* FP:mod.rs-1131 */         if next.0.span.is_dummy() {
/* FP:mod.rs-1132 */             // Tweak the location for better diagnostics, but keep syntactic context intact.
/* FP:mod.rs-1133 */             let fallback_span = self.token.span;
/* FP:mod.rs-1134 */             next.0.span = fallback_span.with_ctxt(next.0.span.ctxt());
/* FP:mod.rs-1135 */         }
/* FP:mod.rs-1136 */         debug_assert!(!matches!(
/* FP:mod.rs-1137 */             next.0.kind,
/* FP:mod.rs-1138 */             token::OpenInvisible(origin) | token::CloseInvisible(origin) if origin.skip()
/* FP:mod.rs-1139 */         ));
/* FP:mod.rs-1140 */         self.inlined_bump_with(next)
/* FP:mod.rs-1141 */     }
/* FP:mod.rs-1142 */ 
/* FP:mod.rs-1143 */     /// Look-ahead `dist` tokens of `self.token` and get access to that token there.
/* FP:mod.rs-1144 */     /// When `dist == 0` then the current token is looked at. `Eof` will be
/* FP:mod.rs-1145 */     /// returned if the look-ahead is any distance past the end of the tokens.
/* FP:mod.rs-1146 */     pub fn look_ahead<R>(&self, dist: usize, looker: impl FnOnce(&Token) -> R) -> R {
/* FP:mod.rs-1147 */         if dist == 0 {
/* FP:mod.rs-1148 */             return looker(&self.token);
/* FP:mod.rs-1149 */         }
/* FP:mod.rs-1150 */ 
/* FP:mod.rs-1151 */         // Typically around 98% of the `dist > 0` cases have `dist == 1`, so we
/* FP:mod.rs-1152 */         // have a fast special case for that.
/* FP:mod.rs-1153 */         if dist == 1 {
/* FP:mod.rs-1154 */             // The index is zero because the tree cursor's index always points
/* FP:mod.rs-1155 */             // to the next token to be gotten.
/* FP:mod.rs-1156 */             match self.token_cursor.curr.curr() {
/* FP:mod.rs-1157 */                 Some(tree) => {
/* FP:mod.rs-1158 */                     // Indexing stayed within the current token tree.
/* FP:mod.rs-1159 */                     match tree {
/* FP:mod.rs-1160 */                         TokenTree::Token(token, _) => return looker(token),
/* FP:mod.rs-1161 */                         &TokenTree::Delimited(dspan, _, delim, _) => {
/* FP:mod.rs-1162 */                             if !delim.skip() {
/* FP:mod.rs-1163 */                                 return looker(&Token::new(delim.as_open_token_kind(), dspan.open));
/* FP:mod.rs-1164 */                             }
/* FP:mod.rs-1165 */                         }
/* FP:mod.rs-1166 */                     }
/* FP:mod.rs-1167 */                 }
/* FP:mod.rs-1168 */                 None => {
/* FP:mod.rs-1169 */                     // The tree cursor lookahead went (one) past the end of the
/* FP:mod.rs-1170 */                     // current token tree. Try to return a close delimiter.
/* FP:mod.rs-1171 */                     if let Some(last) = self.token_cursor.stack.last()
/* FP:mod.rs-1172 */                         && let Some(&TokenTree::Delimited(span, _, delim, _)) = last.curr()
/* FP:mod.rs-1173 */                         && !delim.skip()
/* FP:mod.rs-1174 */                     {
/* FP:mod.rs-1175 */                         // We are not in the outermost token stream, so we have
/* FP:mod.rs-1176 */                         // delimiters. Also, those delimiters are not skipped.
/* FP:mod.rs-1177 */                         return looker(&Token::new(delim.as_close_token_kind(), span.close));
/* FP:mod.rs-1178 */                     }
/* FP:mod.rs-1179 */                 }
/* FP:mod.rs-1180 */             }
/* FP:mod.rs-1181 */         }
/* FP:mod.rs-1182 */ 
/* FP:mod.rs-1183 */         // Just clone the token cursor and use `next`, skipping delimiters as
/* FP:mod.rs-1184 */         // necessary. Slow but simple.
/* FP:mod.rs-1185 */         let mut cursor = self.token_cursor.clone();
/* FP:mod.rs-1186 */         let mut i = 0;
/* FP:mod.rs-1187 */         let mut token = Token::dummy();
/* FP:mod.rs-1188 */         while i < dist {
/* FP:mod.rs-1189 */             token = cursor.next().0;
/* FP:mod.rs-1190 */             if matches!(
/* FP:mod.rs-1191 */                 token.kind,
/* FP:mod.rs-1192 */                 token::OpenInvisible(origin) | token::CloseInvisible(origin) if origin.skip()
/* FP:mod.rs-1193 */             ) {
/* FP:mod.rs-1194 */                 continue;
/* FP:mod.rs-1195 */             }
/* FP:mod.rs-1196 */             i += 1;
/* FP:mod.rs-1197 */         }
/* FP:mod.rs-1198 */         looker(&token)
/* FP:mod.rs-1199 */     }
/* FP:mod.rs-1200 */ 
/* FP:mod.rs-1201 */     /// Like `lookahead`, but skips over token trees rather than tokens. Useful
/* FP:mod.rs-1202 */     /// when looking past possible metavariable pasting sites.
/* FP:mod.rs-1203 */     pub fn tree_look_ahead<R>(
/* FP:mod.rs-1204 */         &self,
/* FP:mod.rs-1205 */         dist: usize,
/* FP:mod.rs-1206 */         looker: impl FnOnce(&TokenTree) -> R,
/* FP:mod.rs-1207 */     ) -> Option<R> {
/* FP:mod.rs-1208 */         assert_ne!(dist, 0);
/* FP:mod.rs-1209 */         self.token_cursor.curr.look_ahead(dist - 1).map(looker)
/* FP:mod.rs-1210 */     }
/* FP:mod.rs-1211 */ 
/* FP:mod.rs-1212 */     /// Returns whether any of the given keywords are `dist` tokens ahead of the current one.
/* FP:mod.rs-1213 */     pub(crate) fn is_keyword_ahead(&self, dist: usize, kws: &[Symbol]) -> bool {
/* FP:mod.rs-1214 */         self.look_ahead(dist, |t| kws.iter().any(|&kw| t.is_keyword(kw)))
/* FP:mod.rs-1215 */     }
/* FP:mod.rs-1216 */ 
/* FP:mod.rs-1217 */     /// Parses asyncness: `async` or nothing.
/* FP:mod.rs-1218 */     fn parse_coroutine_kind(&mut self, case: Case) -> Option<CoroutineKind> {
/* FP:mod.rs-1219 */         let span = self.token_uninterpolated_span();
/* FP:mod.rs-1220 */         if self.eat_keyword_case(exp!(Async), case) {
/* FP:mod.rs-1221 */             // FIXME(gen_blocks): Do we want to unconditionally parse `gen` and then
/* FP:mod.rs-1222 */             // error if edition <= 2024, like we do with async and edition <= 2018?
/* FP:mod.rs-1223 */             if self.token_uninterpolated_span().at_least_rust_2024()
/* FP:mod.rs-1224 */                 && self.eat_keyword_case(exp!(Gen), case)
/* FP:mod.rs-1225 */             {
/* FP:mod.rs-1226 */                 let gen_span = self.prev_token_uninterpolated_span();
/* FP:mod.rs-1227 */                 Some(CoroutineKind::AsyncGen {
/* FP:mod.rs-1228 */                     span: span.to(gen_span),
/* FP:mod.rs-1229 */                     closure_id: DUMMY_NODE_ID,
/* FP:mod.rs-1230 */                     return_impl_trait_id: DUMMY_NODE_ID,
/* FP:mod.rs-1231 */                 })
/* FP:mod.rs-1232 */             } else {
/* FP:mod.rs-1233 */                 Some(CoroutineKind::Async {
/* FP:mod.rs-1234 */                     span,
/* FP:mod.rs-1235 */                     closure_id: DUMMY_NODE_ID,
/* FP:mod.rs-1236 */                     return_impl_trait_id: DUMMY_NODE_ID,
/* FP:mod.rs-1237 */                 })
/* FP:mod.rs-1238 */             }
/* FP:mod.rs-1239 */         } else if self.token_uninterpolated_span().at_least_rust_2024()
/* FP:mod.rs-1240 */             && self.eat_keyword_case(exp!(Gen), case)
/* FP:mod.rs-1241 */         {
/* FP:mod.rs-1242 */             Some(CoroutineKind::Gen {
/* FP:mod.rs-1243 */                 span,
/* FP:mod.rs-1244 */                 closure_id: DUMMY_NODE_ID,
/* FP:mod.rs-1245 */                 return_impl_trait_id: DUMMY_NODE_ID,
/* FP:mod.rs-1246 */             })
/* FP:mod.rs-1247 */         } else {
/* FP:mod.rs-1248 */             None
/* FP:mod.rs-1249 */         }
/* FP:mod.rs-1250 */     }
/* FP:mod.rs-1251 */ 
/* FP:mod.rs-1252 */     /// Parses fn unsafety: `unsafe`, `safe` or nothing.
/* FP:mod.rs-1253 */     fn parse_safety(&mut self, case: Case) -> Safety {
/* FP:mod.rs-1254 */         if self.eat_keyword_case(exp!(Unsafe), case) {
/* FP:mod.rs-1255 */             Safety::Unsafe(self.prev_token_uninterpolated_span())
/* FP:mod.rs-1256 */         } else if self.eat_keyword_case(exp!(Safe), case) {
/* FP:mod.rs-1257 */             Safety::Safe(self.prev_token_uninterpolated_span())
/* FP:mod.rs-1258 */         } else {
/* FP:mod.rs-1259 */             Safety::Default
/* FP:mod.rs-1260 */         }
/* FP:mod.rs-1261 */     }
/* FP:mod.rs-1262 */ 
/* FP:mod.rs-1263 */     /// Parses constness: `const` or nothing.
/* FP:mod.rs-1264 */     fn parse_constness(&mut self, case: Case) -> Const {
/* FP:mod.rs-1265 */         self.parse_constness_(case, false)
/* FP:mod.rs-1266 */     }
/* FP:mod.rs-1267 */ 
/* FP:mod.rs-1268 */     /// Parses constness for closures (case sensitive, feature-gated)
/* FP:mod.rs-1269 */     fn parse_closure_constness(&mut self) -> Const {
/* FP:mod.rs-1270 */         let constness = self.parse_constness_(Case::Sensitive, true);
/* FP:mod.rs-1271 */         if let Const::Yes(span) = constness {
/* FP:mod.rs-1272 */             self.psess.gated_spans.gate(sym::const_closures, span);
/* FP:mod.rs-1273 */         }
/* FP:mod.rs-1274 */         constness
/* FP:mod.rs-1275 */     }
/* FP:mod.rs-1276 */ 
/* FP:mod.rs-1277 */     fn parse_constness_(&mut self, case: Case, is_closure: bool) -> Const {
/* FP:mod.rs-1278 */         // Avoid const blocks and const closures to be parsed as const items
/* FP:mod.rs-1279 */         if (self.check_const_closure() == is_closure)
/* FP:mod.rs-1280 */             && !self.look_ahead(1, |t| *t == token::OpenBrace || t.is_metavar_block())
/* FP:mod.rs-1281 */             && self.eat_keyword_case(exp!(Const), case)
/* FP:mod.rs-1282 */         {
/* FP:mod.rs-1283 */             Const::Yes(self.prev_token_uninterpolated_span())
/* FP:mod.rs-1284 */         } else {
/* FP:mod.rs-1285 */             Const::No
/* FP:mod.rs-1286 */         }
/* FP:mod.rs-1287 */     }
/* FP:mod.rs-1288 */ 
/* FP:mod.rs-1289 */     /// Parses inline const expressions.
/* FP:mod.rs-1290 */     fn parse_const_block(&mut self, span: Span, pat: bool) -> PResult<'a, Box<Expr>> {
/* FP:mod.rs-1291 */         self.expect_keyword(exp!(Const))?;
/* FP:mod.rs-1292 */         let (attrs, blk) = self.parse_inner_attrs_and_block(None)?;
/* FP:mod.rs-1293 */         let anon_const = AnonConst {
/* FP:mod.rs-1294 */             id: DUMMY_NODE_ID,
/* FP:mod.rs-1295 */             value: self.mk_expr(blk.span, ExprKind::Block(blk, None)),
/* FP:mod.rs-1296 */         };
/* FP:mod.rs-1297 */         let blk_span = anon_const.value.span;
/* FP:mod.rs-1298 */         let kind = if pat {
/* FP:mod.rs-1299 */             let guar = self
/* FP:mod.rs-1300 */                 .dcx()
/* FP:mod.rs-1301 */                 .struct_span_err(blk_span, "const blocks cannot be used as patterns")
/* FP:mod.rs-1302 */                 .with_help(
/* FP:mod.rs-1303 */                     "use a named `const`-item or an `if`-guard (`x if x == const { ... }`) instead",
/* FP:mod.rs-1304 */                 )
/* FP:mod.rs-1305 */                 .emit();
/* FP:mod.rs-1306 */             ExprKind::Err(guar)
/* FP:mod.rs-1307 */         } else {
/* FP:mod.rs-1308 */             ExprKind::ConstBlock(anon_const)
/* FP:mod.rs-1309 */         };
/* FP:mod.rs-1310 */         Ok(self.mk_expr_with_attrs(span.to(blk_span), kind, attrs))
/* FP:mod.rs-1311 */     }
/* FP:mod.rs-1312 */ 
/* FP:mod.rs-1313 */     /// Parses mutability (`mut` or nothing).
/* FP:mod.rs-1314 */     fn parse_mutability(&mut self) -> Mutability {
/* FP:mod.rs-1315 */         if self.eat_keyword(exp!(Mut)) { Mutability::Mut } else { Mutability::Not }
/* FP:mod.rs-1316 */     }
/* FP:mod.rs-1317 */ 
/* FP:mod.rs-1318 */     /// Parses reference binding mode (`ref`, `ref mut`, or nothing).
/* FP:mod.rs-1319 */     fn parse_byref(&mut self) -> ByRef {
/* FP:mod.rs-1320 */         if self.eat_keyword(exp!(Ref)) { ByRef::Yes(self.parse_mutability()) } else { ByRef::No }
/* FP:mod.rs-1321 */     }
/* FP:mod.rs-1322 */ 
/* FP:mod.rs-1323 */     /// Possibly parses mutability (`const` or `mut`).
/* FP:mod.rs-1324 */     fn parse_const_or_mut(&mut self) -> Option<Mutability> {
/* FP:mod.rs-1325 */         if self.eat_keyword(exp!(Mut)) {
/* FP:mod.rs-1326 */             Some(Mutability::Mut)
/* FP:mod.rs-1327 */         } else if self.eat_keyword(exp!(Const)) {
/* FP:mod.rs-1328 */             Some(Mutability::Not)
/* FP:mod.rs-1329 */         } else {
/* FP:mod.rs-1330 */             None
/* FP:mod.rs-1331 */         }
/* FP:mod.rs-1332 */     }
/* FP:mod.rs-1333 */ 
/* FP:mod.rs-1334 */     fn parse_field_name(&mut self) -> PResult<'a, Ident> {
/* FP:mod.rs-1335 */         if let token::Literal(token::Lit { kind: token::Integer, symbol, suffix }) = self.token.kind
/* FP:mod.rs-1336 */         {
/* FP:mod.rs-1337 */             if let Some(suffix) = suffix {
/* FP:mod.rs-1338 */                 self.dcx().emit_err(errors::InvalidLiteralSuffixOnTupleIndex {
/* FP:mod.rs-1339 */                     span: self.token.span,
/* FP:mod.rs-1340 */                     suffix,
/* FP:mod.rs-1341 */                 });
/* FP:mod.rs-1342 */             }
/* FP:mod.rs-1343 */             self.bump();
/* FP:mod.rs-1344 */             Ok(Ident::new(symbol, self.prev_token.span))
/* FP:mod.rs-1345 */         } else {
/* FP:mod.rs-1346 */             self.parse_ident_common(true)
/* FP:mod.rs-1347 */         }
/* FP:mod.rs-1348 */     }
/* FP:mod.rs-1349 */ 
/* FP:mod.rs-1350 */     fn parse_delim_args(&mut self) -> PResult<'a, Box<DelimArgs>> {
/* FP:mod.rs-1351 */         if let Some(args) = self.parse_delim_args_inner() {
/* FP:mod.rs-1352 */             Ok(Box::new(args))
/* FP:mod.rs-1353 */         } else {
/* FP:mod.rs-1354 */             self.unexpected_any()
/* FP:mod.rs-1355 */         }
/* FP:mod.rs-1356 */     }
/* FP:mod.rs-1357 */ 
/* FP:mod.rs-1358 */     fn parse_attr_args(&mut self) -> PResult<'a, AttrArgs> {
/* FP:mod.rs-1359 */         Ok(if let Some(args) = self.parse_delim_args_inner() {
/* FP:mod.rs-1360 */             AttrArgs::Delimited(args)
/* FP:mod.rs-1361 */         } else if self.eat(exp!(Eq)) {
/* FP:mod.rs-1362 */             let eq_span = self.prev_token.span;
/* FP:mod.rs-1363 */             let expr = self.parse_expr_force_collect()?;
/* FP:mod.rs-1364 */             AttrArgs::Eq { eq_span, expr }
/* FP:mod.rs-1365 */         } else {
/* FP:mod.rs-1366 */             AttrArgs::Empty
/* FP:mod.rs-1367 */         })
/* FP:mod.rs-1368 */     }
/* FP:mod.rs-1369 */ 
/* FP:mod.rs-1370 */     fn parse_delim_args_inner(&mut self) -> Option<DelimArgs> {
/* FP:mod.rs-1371 */         let delimited = self.check(exp!(OpenParen))
/* FP:mod.rs-1372 */             || self.check(exp!(OpenBracket))
/* FP:mod.rs-1373 */             || self.check(exp!(OpenBrace));
/* FP:mod.rs-1374 */ 
/* FP:mod.rs-1375 */         delimited.then(|| {
/* FP:mod.rs-1376 */             let TokenTree::Delimited(dspan, _, delim, tokens) = self.parse_token_tree() else {
/* FP:mod.rs-1377 */                 unreachable!()
/* FP:mod.rs-1378 */             };
/* FP:mod.rs-1379 */             DelimArgs { dspan, delim, tokens }
/* FP:mod.rs-1380 */         })
/* FP:mod.rs-1381 */     }
/* FP:mod.rs-1382 */ 
/* FP:mod.rs-1383 */     /// Parses a single token tree from the input.
/* FP:mod.rs-1384 */     pub fn parse_token_tree(&mut self) -> TokenTree {
/* FP:mod.rs-1385 */         if self.token.kind.open_delim().is_some() {
/* FP:mod.rs-1386 */             // Clone the `TokenTree::Delimited` that we are currently
/* FP:mod.rs-1387 */             // within. That's what we are going to return.
/* FP:mod.rs-1388 */             let tree = self.token_cursor.stack.last().unwrap().curr().unwrap().clone();
/* FP:mod.rs-1389 */             debug_assert_matches!(tree, TokenTree::Delimited(..));
/* FP:mod.rs-1390 */ 
/* FP:mod.rs-1391 */             // Advance the token cursor through the entire delimited
/* FP:mod.rs-1392 */             // sequence. After getting the `OpenDelim` we are *within* the
/* FP:mod.rs-1393 */             // delimited sequence, i.e. at depth `d`. After getting the
/* FP:mod.rs-1394 */             // matching `CloseDelim` we are *after* the delimited sequence,
/* FP:mod.rs-1395 */             // i.e. at depth `d - 1`.
/* FP:mod.rs-1396 */             let target_depth = self.token_cursor.stack.len() - 1;
/* FP:mod.rs-1397 */ 
/* FP:mod.rs-1398 */             if let Capturing::No = self.capture_state.capturing {
/* FP:mod.rs-1399 */                 // We are not capturing tokens, so skip to the end of the
/* FP:mod.rs-1400 */                 // delimited sequence. This is a perf win when dealing with
/* FP:mod.rs-1401 */                 // declarative macros that pass large `tt` fragments through
/* FP:mod.rs-1402 */                 // multiple rules, as seen in the uom-0.37.0 crate.
/* FP:mod.rs-1403 */                 self.token_cursor.curr.bump_to_end();
/* FP:mod.rs-1404 */                 self.bump();
/* FP:mod.rs-1405 */                 debug_assert_eq!(self.token_cursor.stack.len(), target_depth);
/* FP:mod.rs-1406 */             } else {
/* FP:mod.rs-1407 */                 loop {
/* FP:mod.rs-1408 */                     // Advance one token at a time, so `TokenCursor::next()`
/* FP:mod.rs-1409 */                     // can capture these tokens if necessary.
/* FP:mod.rs-1410 */                     self.bump();
/* FP:mod.rs-1411 */                     if self.token_cursor.stack.len() == target_depth {
/* FP:mod.rs-1412 */                         break;
/* FP:mod.rs-1413 */                     }
/* FP:mod.rs-1414 */                 }
/* FP:mod.rs-1415 */             }
/* FP:mod.rs-1416 */             debug_assert!(self.token.kind.close_delim().is_some());
/* FP:mod.rs-1417 */ 
/* FP:mod.rs-1418 */             // Consume close delimiter
/* FP:mod.rs-1419 */             self.bump();
/* FP:mod.rs-1420 */             tree
/* FP:mod.rs-1421 */         } else {
/* FP:mod.rs-1422 */             assert!(!self.token.kind.is_close_delim_or_eof());
/* FP:mod.rs-1423 */             let prev_spacing = self.token_spacing;
/* FP:mod.rs-1424 */             self.bump();
/* FP:mod.rs-1425 */             TokenTree::Token(self.prev_token, prev_spacing)
/* FP:mod.rs-1426 */         }
/* FP:mod.rs-1427 */     }
/* FP:mod.rs-1428 */ 
/* FP:mod.rs-1429 */     pub fn parse_tokens(&mut self) -> TokenStream {
/* FP:mod.rs-1430 */         let mut result = Vec::new();
/* FP:mod.rs-1431 */         loop {
/* FP:mod.rs-1432 */             if self.token.kind.is_close_delim_or_eof() {
/* FP:mod.rs-1433 */                 break;
/* FP:mod.rs-1434 */             } else {
/* FP:mod.rs-1435 */                 result.push(self.parse_token_tree());
/* FP:mod.rs-1436 */             }
/* FP:mod.rs-1437 */         }
/* FP:mod.rs-1438 */         TokenStream::new(result)
/* FP:mod.rs-1439 */     }
/* FP:mod.rs-1440 */ 
/* FP:mod.rs-1441 */     /// Evaluates the closure with restrictions in place.
/* FP:mod.rs-1442 */     ///
/* FP:mod.rs-1443 */     /// Afters the closure is evaluated, restrictions are reset.
/* FP:mod.rs-1444 */     fn with_res<T>(&mut self, res: Restrictions, f: impl FnOnce(&mut Self) -> T) -> T {
/* FP:mod.rs-1445 */         let old = self.restrictions;
/* FP:mod.rs-1446 */         self.restrictions = res;
/* FP:mod.rs-1447 */         let res = f(self);
/* FP:mod.rs-1448 */         self.restrictions = old;
/* FP:mod.rs-1449 */         res
/* FP:mod.rs-1450 */     }
/* FP:mod.rs-1451 */ 
/* FP:mod.rs-1452 */     /// Parses `pub` and `pub(in path)` plus shortcuts `pub(crate)` for `pub(in crate)`, `pub(self)`
/* FP:mod.rs-1453 */     /// for `pub(in self)` and `pub(super)` for `pub(in super)`.
/* FP:mod.rs-1454 */     /// If the following element can't be a tuple (i.e., it's a function definition), then
/* FP:mod.rs-1455 */     /// it's not a tuple struct field), and the contents within the parentheses aren't valid,
/* FP:mod.rs-1456 */     /// so emit a proper diagnostic.
/* FP:mod.rs-1457 */     // Public for rustfmt usage.
/* FP:mod.rs-1458 */     pub fn parse_visibility(&mut self, fbt: FollowedByType) -> PResult<'a, Visibility> {
/* FP:mod.rs-1459 */         if let Some(vis) = self
/* FP:mod.rs-1460 */             .eat_metavar_seq(MetaVarKind::Vis, |this| this.parse_visibility(FollowedByType::Yes))
/* FP:mod.rs-1461 */         {
/* FP:mod.rs-1462 */             return Ok(vis);
/* FP:mod.rs-1463 */         }
/* FP:mod.rs-1464 */ 
/* FP:mod.rs-1465 */         if !self.eat_keyword(exp!(Pub)) {
/* FP:mod.rs-1466 */             // We need a span for our `Spanned<VisibilityKind>`, but there's inherently no
/* FP:mod.rs-1467 */             // keyword to grab a span from for inherited visibility; an empty span at the
/* FP:mod.rs-1468 */             // beginning of the current token would seem to be the "Schelling span".
/* FP:mod.rs-1469 */             return Ok(Visibility {
/* FP:mod.rs-1470 */                 span: self.token.span.shrink_to_lo(),
/* FP:mod.rs-1471 */                 kind: VisibilityKind::Inherited,
/* FP:mod.rs-1472 */                 tokens: None,
/* FP:mod.rs-1473 */             });
/* FP:mod.rs-1474 */         }
/* FP:mod.rs-1475 */         let lo = self.prev_token.span;
/* FP:mod.rs-1476 */ 
/* FP:mod.rs-1477 */         if self.check(exp!(OpenParen)) {
/* FP:mod.rs-1478 */             // We don't `self.bump()` the `(` yet because this might be a struct definition where
/* FP:mod.rs-1479 */             // `()` or a tuple might be allowed. For example, `struct Struct(pub (), pub (usize));`.
/* FP:mod.rs-1480 */             // Because of this, we only `bump` the `(` if we're assured it is appropriate to do so
/* FP:mod.rs-1481 */             // by the following tokens.
/* FP:mod.rs-1482 */             if self.is_keyword_ahead(1, &[kw::In]) {
/* FP:mod.rs-1483 */                 // Parse `pub(in path)`.
/* FP:mod.rs-1484 */                 self.bump(); // `(`
/* FP:mod.rs-1485 */                 self.bump(); // `in`
/* FP:mod.rs-1486 */                 let path = self.parse_path(PathStyle::Mod)?; // `path`
/* FP:mod.rs-1487 */                 self.expect(exp!(CloseParen))?; // `)`
/* FP:mod.rs-1488 */                 let vis = VisibilityKind::Restricted {
/* FP:mod.rs-1489 */                     path: Box::new(path),
/* FP:mod.rs-1490 */                     id: ast::DUMMY_NODE_ID,
/* FP:mod.rs-1491 */                     shorthand: false,
/* FP:mod.rs-1492 */                 };
/* FP:mod.rs-1493 */                 return Ok(Visibility {
/* FP:mod.rs-1494 */                     span: lo.to(self.prev_token.span),
/* FP:mod.rs-1495 */                     kind: vis,
/* FP:mod.rs-1496 */                     tokens: None,
/* FP:mod.rs-1497 */                 });
/* FP:mod.rs-1498 */             } else if self.look_ahead(2, |t| t == &token::CloseParen)
/* FP:mod.rs-1499 */                 && self.is_keyword_ahead(1, &[kw::Crate, kw::Super, kw::SelfLower])
/* FP:mod.rs-1500 */             {
/* FP:mod.rs-1501 */                 // Parse `pub(crate)`, `pub(self)`, or `pub(super)`.
/* FP:mod.rs-1502 */                 self.bump(); // `(`
/* FP:mod.rs-1503 */                 let path = self.parse_path(PathStyle::Mod)?; // `crate`/`super`/`self`
/* FP:mod.rs-1504 */                 self.expect(exp!(CloseParen))?; // `)`
/* FP:mod.rs-1505 */                 let vis = VisibilityKind::Restricted {
/* FP:mod.rs-1506 */                     path: Box::new(path),
/* FP:mod.rs-1507 */                     id: ast::DUMMY_NODE_ID,
/* FP:mod.rs-1508 */                     shorthand: true,
/* FP:mod.rs-1509 */                 };
/* FP:mod.rs-1510 */                 return Ok(Visibility {
/* FP:mod.rs-1511 */                     span: lo.to(self.prev_token.span),
/* FP:mod.rs-1512 */                     kind: vis,
/* FP:mod.rs-1513 */                     tokens: None,
/* FP:mod.rs-1514 */                 });
/* FP:mod.rs-1515 */             } else if let FollowedByType::No = fbt {
/* FP:mod.rs-1516 */                 // Provide this diagnostic if a type cannot follow;
/* FP:mod.rs-1517 */                 // in particular, if this is not a tuple struct.
/* FP:mod.rs-1518 */                 self.recover_incorrect_vis_restriction()?;
/* FP:mod.rs-1519 */                 // Emit diagnostic, but continue with public visibility.
/* FP:mod.rs-1520 */             }
/* FP:mod.rs-1521 */         }
/* FP:mod.rs-1522 */ 
/* FP:mod.rs-1523 */         Ok(Visibility { span: lo, kind: VisibilityKind::Public, tokens: None })
/* FP:mod.rs-1524 */     }
/* FP:mod.rs-1525 */ 
/* FP:mod.rs-1526 */     /// Recovery for e.g. `pub(something) fn ...` or `struct X { pub(something) y: Z }`
/* FP:mod.rs-1527 */     fn recover_incorrect_vis_restriction(&mut self) -> PResult<'a, ()> {
/* FP:mod.rs-1528 */         self.bump(); // `(`
/* FP:mod.rs-1529 */         let path = self.parse_path(PathStyle::Mod)?;
/* FP:mod.rs-1530 */         self.expect(exp!(CloseParen))?; // `)`
/* FP:mod.rs-1531 */ 
/* FP:mod.rs-1532 */         let path_str = pprust::path_to_string(&path);
/* FP:mod.rs-1533 */         self.dcx()
/* FP:mod.rs-1534 */             .emit_err(IncorrectVisibilityRestriction { span: path.span, inner_str: path_str });
/* FP:mod.rs-1535 */ 
/* FP:mod.rs-1536 */         Ok(())
/* FP:mod.rs-1537 */     }
/* FP:mod.rs-1538 */ 
/* FP:mod.rs-1539 */     /// Parses `extern string_literal?`.
/* FP:mod.rs-1540 */     fn parse_extern(&mut self, case: Case) -> Extern {
/* FP:mod.rs-1541 */         if self.eat_keyword_case(exp!(Extern), case) {
/* FP:mod.rs-1542 */             let mut extern_span = self.prev_token.span;
/* FP:mod.rs-1543 */             let abi = self.parse_abi();
/* FP:mod.rs-1544 */             if let Some(abi) = abi {
/* FP:mod.rs-1545 */                 extern_span = extern_span.to(abi.span);
/* FP:mod.rs-1546 */             }
/* FP:mod.rs-1547 */             Extern::from_abi(abi, extern_span)
/* FP:mod.rs-1548 */         } else {
/* FP:mod.rs-1549 */             Extern::None
/* FP:mod.rs-1550 */         }
/* FP:mod.rs-1551 */     }
/* FP:mod.rs-1552 */ 
/* FP:mod.rs-1553 */     /// Parses a string literal as an ABI spec.
/* FP:mod.rs-1554 */     fn parse_abi(&mut self) -> Option<StrLit> {
/* FP:mod.rs-1555 */         match self.parse_str_lit() {
/* FP:mod.rs-1556 */             Ok(str_lit) => Some(str_lit),
/* FP:mod.rs-1557 */             Err(Some(lit)) => match lit.kind {
/* FP:mod.rs-1558 */                 ast::LitKind::Err(_) => None,
/* FP:mod.rs-1559 */                 _ => {
/* FP:mod.rs-1560 */                     self.dcx().emit_err(NonStringAbiLiteral { span: lit.span });
/* FP:mod.rs-1561 */                     None
/* FP:mod.rs-1562 */                 }
/* FP:mod.rs-1563 */             },
/* FP:mod.rs-1564 */             Err(None) => None,
/* FP:mod.rs-1565 */         }
/* FP:mod.rs-1566 */     }
/* FP:mod.rs-1567 */ 
/* FP:mod.rs-1568 */     fn collect_tokens_no_attrs<R: HasAttrs + HasTokens>(
/* FP:mod.rs-1569 */         &mut self,
/* FP:mod.rs-1570 */         f: impl FnOnce(&mut Self) -> PResult<'a, R>,
/* FP:mod.rs-1571 */     ) -> PResult<'a, R> {
/* FP:mod.rs-1572 */         // The only reason to call `collect_tokens_no_attrs` is if you want tokens, so use
/* FP:mod.rs-1573 */         // `ForceCollect::Yes`
/* FP:mod.rs-1574 */         self.collect_tokens(None, AttrWrapper::empty(), ForceCollect::Yes, |this, _attrs| {
/* FP:mod.rs-1575 */             Ok((f(this)?, Trailing::No, UsePreAttrPos::No))
/* FP:mod.rs-1576 */         })
/* FP:mod.rs-1577 */     }
/* FP:mod.rs-1578 */ 
/* FP:mod.rs-1579 */     /// Checks for `::` or, potentially, `:::` and then look ahead after it.
/* FP:mod.rs-1580 */     fn check_path_sep_and_look_ahead(&mut self, looker: impl Fn(&Token) -> bool) -> bool {
/* FP:mod.rs-1581 */         if self.check(exp!(PathSep)) {
/* FP:mod.rs-1582 */             if self.may_recover() && self.look_ahead(1, |t| t.kind == token::Colon) {
/* FP:mod.rs-1583 */                 debug_assert!(!self.look_ahead(1, &looker), "Looker must not match on colon");
/* FP:mod.rs-1584 */                 self.look_ahead(2, looker)
/* FP:mod.rs-1585 */             } else {
/* FP:mod.rs-1586 */                 self.look_ahead(1, looker)
/* FP:mod.rs-1587 */             }
/* FP:mod.rs-1588 */         } else {
/* FP:mod.rs-1589 */             false
/* FP:mod.rs-1590 */         }
/* FP:mod.rs-1591 */     }
/* FP:mod.rs-1592 */ 
/* FP:mod.rs-1593 */     /// `::{` or `::*`
/* FP:mod.rs-1594 */     fn is_import_coupler(&mut self) -> bool {
/* FP:mod.rs-1595 */         self.check_path_sep_and_look_ahead(|t| matches!(t.kind, token::OpenBrace | token::Star))
/* FP:mod.rs-1596 */     }
/* FP:mod.rs-1597 */ 
/* FP:mod.rs-1598 */     // Debug view of the parser's token stream, up to `{lookahead}` tokens.
/* FP:mod.rs-1599 */     // Only used when debugging.
/* FP:mod.rs-1600 */     #[allow(unused)]
/* FP:mod.rs-1601 */     pub(crate) fn debug_lookahead(&self, lookahead: usize) -> impl fmt::Debug {
/* FP:mod.rs-1602 */         fmt::from_fn(move |f| {
/* FP:mod.rs-1603 */             let mut dbg_fmt = f.debug_struct("Parser"); // or at least, one view of
/* FP:mod.rs-1604 */ 
/* FP:mod.rs-1605 */             // we don't need N spans, but we want at least one, so print all of prev_token
/* FP:mod.rs-1606 */             dbg_fmt.field("prev_token", &self.prev_token);
/* FP:mod.rs-1607 */             let mut tokens = vec![];
/* FP:mod.rs-1608 */             for i in 0..lookahead {
/* FP:mod.rs-1609 */                 let tok = self.look_ahead(i, |tok| tok.kind);
/* FP:mod.rs-1610 */                 let is_eof = tok == TokenKind::Eof;
/* FP:mod.rs-1611 */                 tokens.push(tok);
/* FP:mod.rs-1612 */                 if is_eof {
/* FP:mod.rs-1613 */                     // Don't look ahead past EOF.
/* FP:mod.rs-1614 */                     break;
/* FP:mod.rs-1615 */                 }
/* FP:mod.rs-1616 */             }
/* FP:mod.rs-1617 */             dbg_fmt.field_with("tokens", |field| field.debug_list().entries(tokens).finish());
/* FP:mod.rs-1618 */             dbg_fmt.field("approx_token_stream_pos", &self.num_bump_calls);
/* FP:mod.rs-1619 */ 
/* FP:mod.rs-1620 */             // some fields are interesting for certain values, as they relate to macro parsing
/* FP:mod.rs-1621 */             if let Some(subparser) = self.subparser_name {
/* FP:mod.rs-1622 */                 dbg_fmt.field("subparser_name", &subparser);
/* FP:mod.rs-1623 */             }
/* FP:mod.rs-1624 */             if let Recovery::Forbidden = self.recovery {
/* FP:mod.rs-1625 */                 dbg_fmt.field("recovery", &self.recovery);
/* FP:mod.rs-1626 */             }
/* FP:mod.rs-1627 */ 
/* FP:mod.rs-1628 */             // imply there's "more to know" than this view
/* FP:mod.rs-1629 */             dbg_fmt.finish_non_exhaustive()
/* FP:mod.rs-1630 */         })
/* FP:mod.rs-1631 */     }
/* FP:mod.rs-1632 */ 
/* FP:mod.rs-1633 */     pub fn clear_expected_token_types(&mut self) {
/* FP:mod.rs-1634 */         self.expected_token_types.clear();
/* FP:mod.rs-1635 */     }
/* FP:mod.rs-1636 */ 
/* FP:mod.rs-1637 */     pub fn approx_token_stream_pos(&self) -> u32 {
/* FP:mod.rs-1638 */         self.num_bump_calls
/* FP:mod.rs-1639 */     }
/* FP:mod.rs-1640 */ 
/* FP:mod.rs-1641 */     /// For interpolated `self.token`, returns a span of the fragment to which
/* FP:mod.rs-1642 */     /// the interpolated token refers. For all other tokens this is just a
/* FP:mod.rs-1643 */     /// regular span. It is particularly important to use this for identifiers
/* FP:mod.rs-1644 */     /// and lifetimes for which spans affect name resolution and edition
/* FP:mod.rs-1645 */     /// checks. Note that keywords are also identifiers, so they should use
/* FP:mod.rs-1646 */     /// this if they keep spans or perform edition checks.
/* FP:mod.rs-1647 */     pub fn token_uninterpolated_span(&self) -> Span {
/* FP:mod.rs-1648 */         match &self.token.kind {
/* FP:mod.rs-1649 */             token::NtIdent(ident, _) | token::NtLifetime(ident, _) => ident.span,
/* FP:mod.rs-1650 */             token::OpenInvisible(InvisibleOrigin::MetaVar(_)) => self.look_ahead(1, |t| t.span),
/* FP:mod.rs-1651 */             _ => self.token.span,
/* FP:mod.rs-1652 */         }
/* FP:mod.rs-1653 */     }
/* FP:mod.rs-1654 */ 
/* FP:mod.rs-1655 */     /// Like `token_uninterpolated_span`, but works on `self.prev_token`.
/* FP:mod.rs-1656 */     pub fn prev_token_uninterpolated_span(&self) -> Span {
/* FP:mod.rs-1657 */         match &self.prev_token.kind {
/* FP:mod.rs-1658 */             token::NtIdent(ident, _) | token::NtLifetime(ident, _) => ident.span,
/* FP:mod.rs-1659 */             token::OpenInvisible(InvisibleOrigin::MetaVar(_)) => self.look_ahead(0, |t| t.span),
/* FP:mod.rs-1660 */             _ => self.prev_token.span,
/* FP:mod.rs-1661 */         }
/* FP:mod.rs-1662 */     }
/* FP:mod.rs-1663 */ }
/* FP:mod.rs-1664 */ 
/* FP:mod.rs-1665 */ // Metavar captures of various kinds.
/* FP:mod.rs-1666 */ #[derive(Clone, Debug)]
/* FP:mod.rs-1667 */ pub enum ParseNtResult {
/* FP:mod.rs-1668 */     Tt(TokenTree),
/* FP:mod.rs-1669 */     Ident(Ident, IdentIsRaw),
/* FP:mod.rs-1670 */     Lifetime(Ident, IdentIsRaw),
/* FP:mod.rs-1671 */     Item(Box<ast::Item>),
/* FP:mod.rs-1672 */     Block(Box<ast::Block>),
/* FP:mod.rs-1673 */     Stmt(Box<ast::Stmt>),
/* FP:mod.rs-1674 */     Pat(Box<ast::Pat>, NtPatKind),
/* FP:mod.rs-1675 */     Expr(Box<ast::Expr>, NtExprKind),
/* FP:mod.rs-1676 */     Literal(Box<ast::Expr>),
/* FP:mod.rs-1677 */     Ty(Box<ast::Ty>),
/* FP:mod.rs-1678 */     Meta(Box<ast::AttrItem>),
/* FP:mod.rs-1679 */     Path(Box<ast::Path>),
/* FP:mod.rs-1680 */     Vis(Box<ast::Visibility>),
/* FP:mod.rs-1681 */ }