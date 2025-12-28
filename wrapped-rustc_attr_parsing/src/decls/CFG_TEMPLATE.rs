macro_rules! CFG_TEMPLATE {
    () => {
        pub const CFG_TEMPLATE : AttributeTemplate = template ! (List : & ["predicate"] , "https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg-attribute") ;
    };
}

CFG_TEMPLATE!()