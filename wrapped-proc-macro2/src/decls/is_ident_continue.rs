macro_rules! is_ident_continue {
    () => {
        pub (crate) fn is_ident_continue (c : char) -> bool { unicode_ident :: is_xid_continue (c) }
    };
}

is_ident_continue!()