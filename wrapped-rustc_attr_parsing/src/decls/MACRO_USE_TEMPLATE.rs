macro_rules! MACRO_USE_TEMPLATE {
    () => {
        const MACRO_USE_TEMPLATE : AttributeTemplate = template ! (Word , List : & ["name1, name2, ..."] , "https://doc.rust-lang.org/reference/macros-by-example.html#the-macro_use-attribute") ;
    };
}

MACRO_USE_TEMPLATE!();