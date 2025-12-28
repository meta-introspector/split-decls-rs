macro_rules! Transparent {
    () => {
        # [derive (Copy , Clone)] pub struct Transparent < 'a > { pub original : & 'a Attribute , pub span : Span , }
    };
}

Transparent!();