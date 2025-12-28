macro_rules! do_mock {
    () => {
        fn do_mock (input : TokenStream) -> TokenStream { cfg_if ! { if # [cfg (reprocheck)] { let ts_a = do_mock_once (input . clone ()) ; let ts_b = do_mock_once (input . clone ()) ; assert_eq ! (ts_a . to_string () , ts_b . to_string ()) ; } } do_mock_once (input) }
    };
}

do_mock!()