macro_rules! is_builtin_attr_name {
    () => {
        pub fn is_builtin_attr_name (name : Symbol) -> bool { BUILTIN_ATTRIBUTE_MAP . get (& name) . is_some () }
    };
}

is_builtin_attr_name!()