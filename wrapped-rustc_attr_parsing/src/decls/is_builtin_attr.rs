macro_rules! is_builtin_attr {
    () => {
        pub fn is_builtin_attr (attr : & impl AttributeExt) -> bool { attr . is_doc_comment () || attr . ident () . is_some_and (| ident | is_builtin_attr_name (ident . name)) }
    };
}

is_builtin_attr!();