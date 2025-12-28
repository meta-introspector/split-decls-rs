macro_rules! mock_it {
    () => {
        fn mock_it < M : Into < MockableItem > > (inputs : M) -> TokenStream { let mockable : MockableItem = inputs . into () ; let mock = MockItem :: from (mockable) ; let ts = mock . into_token_stream () ; if env :: var ("MOCKALL_DEBUG") . is_ok () { println ! ("{ts}") ; } ts }
    };
}

mock_it!()