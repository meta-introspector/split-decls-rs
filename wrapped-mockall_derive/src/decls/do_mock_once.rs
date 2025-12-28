macro_rules! do_mock_once {
    () => {
        fn do_mock_once (input : TokenStream) -> TokenStream { let item : MockableStruct = match syn :: parse2 (input) { Ok (mock) => mock , Err (err) => { return err . to_compile_error () ; } } ; mock_it (item) }
    };
}

do_mock_once!()