macro_rules! check_builtin_macro_attribute {
    () => {
        pub (crate) fn check_builtin_macro_attribute (ecx : & ExtCtxt < '_ > , meta_item : & MetaItem , name : Symbol) { let template = AttributeTemplate { word : true , .. Default :: default () } ; validate_attr :: check_builtin_meta_item (& ecx . sess . psess , meta_item , AttrStyle :: Outer , name , template , true ,) ; }
    };
}

check_builtin_macro_attribute!()