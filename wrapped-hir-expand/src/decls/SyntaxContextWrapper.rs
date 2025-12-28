macro_rules! SyntaxContextWrapper {
    () => {
        # [salsa_macros :: interned (no_lifetime , id = span :: SyntaxContext , revisions = usize :: MAX)] pub struct SyntaxContextWrapper { pub data : SyntaxContext , }
    };
}

SyntaxContextWrapper!();