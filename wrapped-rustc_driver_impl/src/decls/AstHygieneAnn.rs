macro_rules! AstHygieneAnn {
    () => {
        struct AstHygieneAnn < 'a > { sess : & 'a Session , }
    };
}

AstHygieneAnn!()