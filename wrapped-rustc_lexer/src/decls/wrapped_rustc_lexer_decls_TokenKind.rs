use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Enum representing common lexeme types.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    /// A line comment, e.g. `// comment`.
    LineComment { doc_style: Option<DocStyle> },
    /// A block comment, e.g. `/* block comment */`.
    ///
    /// Block comments can be recursive, so a sequence like `/* /* */`
    /// will not be considered terminated and will result in a parsing error.
    BlockComment { doc_style: Option<DocStyle>, terminated: bool },
    /// Any whitespace character sequence.
    Whitespace,
    Frontmatter { has_invalid_preceding_whitespace: bool, invalid_infostring: bool },
    /// An identifier or keyword, e.g. `ident` or `continue`.
    Ident,
    /// An identifier that is invalid because it contains emoji.
    InvalidIdent,
    /// A raw identifier, e.g. "r#ident".
    RawIdent,
    /// An unknown literal prefix, like `foo#`, `foo'`, `foo"`. Excludes
    /// literal prefixes that contain emoji, which are considered "invalid".
    ///
    /// Note that only the
    /// prefix (`foo`) is included in the token, not the separator (which is
    /// lexed as its own distinct token). In Rust 2021 and later, reserved
    /// prefixes are reported as errors; in earlier editions, they result in a
    /// (allowed by default) lint, and are treated as regular identifier
    /// tokens.
    UnknownPrefix,
    /// An unknown prefix in a lifetime, like `'foo#`.
    ///
    /// Like `UnknownPrefix`, only the `'` and prefix are included in the token
    /// and not the separator.
    UnknownPrefixLifetime,
    /// A raw lifetime, e.g. `'r#foo`. In edition < 2021 it will be split into
    /// several tokens: `'r` and `#` and `foo`.
    RawLifetime,
    /// Guarded string literal prefix: `#"` or `##`.
    ///
    /// Used for reserving "guarded strings" (RFC 3598) in edition 2024.
    /// Split into the component tokens on older editions.
    GuardedStrPrefix,
    /// Literals, e.g. `12u8`, `1.0e-40`, `b"123"`. Note that `_` is an invalid
    /// suffix, but may be present here on string and float literals. Users of
    /// this type will need to check for and reject that case.
    ///
    /// See [LiteralKind] for more details.
    Literal { kind: LiteralKind, suffix_start: u32 },
    /// A lifetime, e.g. `'a`.
    Lifetime { starts_with_number: bool },
    /// `;`
    Semi,
    /// `,`
    Comma,
    /// `.`
    Dot,
    /// `(`
    OpenParen,
    /// `)`
    CloseParen,
    /// `{`
    OpenBrace,
    /// `}`
    CloseBrace,
    /// `[`
    OpenBracket,
    /// `]`
    CloseBracket,
    /// `@`
    At,
    /// `#`
    Pound,
    /// `~`
    Tilde,
    /// `?`
    Question,
    /// `:`
    Colon,
    /// `$`
    Dollar,
    /// `=`
    Eq,
    /// `!`
    Bang,
    /// `<`
    Lt,
    /// `>`
    Gt,
    /// `-`
    Minus,
    /// `&`
    And,
    /// `|`
    Or,
    /// `+`
    Plus,
    /// `*`
    Star,
    /// `/`
    Slash,
    /// `^`
    Caret,
    /// `%`
    Percent,
    /// Unknown token, not expected by the lexer, e.g. "№"
    Unknown,
    /// End of input.
    Eof,
}
