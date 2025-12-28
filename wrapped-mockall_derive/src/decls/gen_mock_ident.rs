macro_rules! gen_mock_ident {
    () => {
        # [doc = " Generate a mock identifier from the regular one: eg \"Foo\" => \"MockFoo\""] fn gen_mock_ident (ident : & Ident) -> Ident { format_ident ! ("Mock{}" , ident) }
    };
}

gen_mock_ident!();