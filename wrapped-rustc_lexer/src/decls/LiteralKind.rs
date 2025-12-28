macro_rules! deps {
    () => {
        Base!();
    };
}

macro_rules! LiteralKind {
    () => {
        deps!();
        # [doc = " Enum representing the literal types supported by the lexer."] # [doc = ""] # [doc = " Note that the suffix is *not* considered when deciding the `LiteralKind` in"] # [doc = " this type. This means that float literals like `1f32` are classified by this"] # [doc = " type as `Int`. (Compare against `rustc_ast::token::LitKind` and"] # [doc = " `rustc_ast::ast::LitKind`)."] # [derive (Clone , Copy , Debug , PartialEq , Eq , PartialOrd , Ord)] pub enum LiteralKind { # [doc = " `12_u8`, `0o100`, `0b120i99`, `1f32`."] Int { base : Base , empty_int : bool } , # [doc = " `12.34f32`, `1e3`, but not `1f32`."] Float { base : Base , empty_exponent : bool } , # [doc = " `'a'`, `'\\\\'`, `'''`, `';`"] Char { terminated : bool } , # [doc = " `b'a'`, `b'\\\\'`, `b'''`, `b';`"] Byte { terminated : bool } , # [doc = " `\"abc\"`, `\"abc`"] Str { terminated : bool } , # [doc = " `b\"abc\"`, `b\"abc`"] ByteStr { terminated : bool } , # [doc = " `c\"abc\"`, `c\"abc`"] CStr { terminated : bool } , # [doc = " `r\"abc\"`, `r#\"abc\"#`, `r####\"ab\"###\"c\"####`, `r#\"a`. `None` indicates"] # [doc = " an invalid literal."] RawStr { n_hashes : Option < u8 > } , # [doc = " `br\"abc\"`, `br#\"abc\"#`, `br####\"ab\"###\"c\"####`, `br#\"a`. `None`"] # [doc = " indicates an invalid literal."] RawByteStr { n_hashes : Option < u8 > } , # [doc = " `cr\"abc\"`, \"cr#\"abc\"#\", `cr#\"a`. `None` indicates an invalid literal."] RawCStr { n_hashes : Option < u8 > } , }
    };
}

LiteralKind!();