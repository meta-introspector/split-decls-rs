macro_rules! deps {
    () => {
        InnerItemLinter!();
    };
}

macro_rules! impl_339 {
    () => {
        deps!();
        impl < 'a > Visitor < 'a > for InnerItemLinter < '_ > { fn visit_item (& mut self , i : & 'a ast :: Item) { if let Some (attr) = attr :: find_by_name (& i . attrs , sym :: rustc_test_marker) { self . sess . psess . buffer_lint (UNNAMEABLE_TEST_ITEMS , attr . span , i . id , BuiltinLintDiag :: UnnameableTestItems ,) ; } } }
    };
}

impl_339!()