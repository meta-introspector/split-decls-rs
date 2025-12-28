macro_rules! suffixed_ident {
    () => {
        fn suffixed_ident (name : & str , suffix : usize , s : Span) -> Ident { Ident :: new (& format ! ("{name}_{suffix}") , s) }
    };
}

suffixed_ident!()