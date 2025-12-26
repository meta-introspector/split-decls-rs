use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Enum representing the literal types supported by the lexer.
///
/// Note that the suffix is *not* considered when deciding the `LiteralKind` in
/// this type. This means that float literals like `1f32` are classified by this
/// type as `Int`. (Compare against `rustc_ast::token::LitKind` and
/// `rustc_ast::ast::LitKind`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LiteralKind {
    /// `12_u8`, `0o100`, `0b120i99`, `1f32`.
    Int { base: Base, empty_int: bool },
    /// `12.34f32`, `1e3`, but not `1f32`.
    Float { base: Base, empty_exponent: bool },
    /// `'a'`, `'\\'`, `'''`, `';`
    Char { terminated: bool },
    /// `b'a'`, `b'\\'`, `b'''`, `b';`
    Byte { terminated: bool },
    /// `"abc"`, `"abc`
    Str { terminated: bool },
    /// `b"abc"`, `b"abc`
    ByteStr { terminated: bool },
    /// `c"abc"`, `c"abc`
    CStr { terminated: bool },
    /// `r"abc"`, `r#"abc"#`, `r####"ab"###"c"####`, `r#"a`. `None` indicates
    /// an invalid literal.
    RawStr { n_hashes: Option<u8> },
    /// `br"abc"`, `br#"abc"#`, `br####"ab"###"c"####`, `br#"a`. `None`
    /// indicates an invalid literal.
    RawByteStr { n_hashes: Option<u8> },
    /// `cr"abc"`, "cr#"abc"#", `cr#"a`. `None` indicates an invalid literal.
    RawCStr { n_hashes: Option<u8> },
}
