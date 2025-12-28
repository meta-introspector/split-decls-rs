macro_rules! Source {
    () => {
        # [derive (Copy , Clone)] pub struct Source < 'a > { pub original : & 'a Attribute , pub span : Span , }
    };
}

Source!()