macro_rules! deps {
    () => {
        InnerItemLinter!();
        TestHarnessGenerator!();
        Test!();
    };
}

macro_rules! impl_337 {
    () => {
        deps!();
        impl < 'a > MutVisitor for TestHarnessGenerator < 'a > { fn visit_crate (& mut self , c : & mut ast :: Crate) { let prev_tests = mem :: take (& mut self . tests) ; walk_crate (self , c) ; self . add_test_cases (ast :: CRATE_NODE_ID , c . spans . inner_span , prev_tests) ; c . items . push (mk_main (& mut self . cx)) ; } fn visit_item (& mut self , item : & mut ast :: Item) { if let Some (name) = get_test_name (& item) { debug ! ("this is a test item") ; let test = Test { span : item . span , ident : item . kind . ident () . unwrap () , name } ; self . tests . push (test) ; } if let ast :: ItemKind :: Mod (_ , _ , ModKind :: Loaded (.. , ast :: ModSpans { inner_span : span , .. }) ,) = item . kind { let prev_tests = mem :: take (& mut self . tests) ; ast :: mut_visit :: walk_item (self , item) ; self . add_test_cases (item . id , span , prev_tests) ; } else { ast :: visit :: walk_item (& mut InnerItemLinter { sess : self . cx . ext_cx . sess } , & item) ; } } }
    };
}

impl_337!()