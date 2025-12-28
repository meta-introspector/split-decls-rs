macro_rules! expand_test {
    () => {
        pub (crate) fn expand_test (cx : & mut ExtCtxt < '_ > , attr_sp : Span , meta_item : & ast :: MetaItem , item : Annotatable ,) -> Vec < Annotatable > { check_builtin_macro_attribute (cx , meta_item , sym :: test) ; warn_on_duplicate_attribute (cx , & item , sym :: test) ; expand_test_or_bench (cx , attr_sp , item , false) }
    };
}

expand_test!()