macro_rules! subty_if_name {
    () => {
        pub (crate) fn subty_if_name < 'a > (ty : & 'a Type , name : & str) -> Option < & 'a Type > { subty_if (ty , | seg | seg . ident == name) }
    };
}

subty_if_name!()