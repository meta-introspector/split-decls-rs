macro_rules! deps {
    () => {
        TestHarnessGenerator!();
        Test!();
    };
}

macro_rules! impl_336 {
    () => {
        deps!();
        impl TestHarnessGenerator < '_ > { fn add_test_cases (& mut self , node_id : ast :: NodeId , span : Span , prev_tests : Vec < Test >) { let mut tests = mem :: replace (& mut self . tests , prev_tests) ; if ! tests . is_empty () { let expn_id = self . cx . ext_cx . resolver . expansion_for_ast_pass (span , AstPass :: TestHarness , & [] , Some (node_id) ,) ; for test in & mut tests { test . ident . span = test . ident . span . apply_mark (expn_id . to_expn_id () , Transparency :: Opaque) ; } self . cx . test_cases . extend (tests) ; } } }
    };
}

impl_336!()