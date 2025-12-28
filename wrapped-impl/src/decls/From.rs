macro_rules! From {
    () => {
        # [derive (Copy , Clone)] pub struct From < 'a > { pub original : & 'a Attribute , pub span : Span , }
    };
}

From!()