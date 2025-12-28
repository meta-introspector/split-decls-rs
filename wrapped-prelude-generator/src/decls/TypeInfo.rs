macro_rules! TypeInfo {
    () => {
        pub struct TypeInfo { pub count : usize , pub layer : Option < usize > , }
    };
}

TypeInfo!();