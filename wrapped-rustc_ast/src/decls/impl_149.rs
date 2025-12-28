macro_rules! deps {
    () => {
        InlineAsmTemplatePiece!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        impl InlineAsmTemplatePiece { # [doc = " Rebuilds the asm template string from its pieces."] pub fn to_string (s : & [Self]) -> String { use fmt :: Write ; let mut out = String :: new () ; for p in s . iter () { let _ = write ! (out , "{p}") ; } out } }
    };
}

impl_149!()