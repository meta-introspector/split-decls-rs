macro_rules! deps {
    () => {
        Runnable!();
    };
}

macro_rules! related_tests {
    () => {
        deps!();
        pub (crate) fn related_tests (db : & RootDatabase , position : FilePosition , search_scope : Option < SearchScope > ,) -> Vec < Runnable > { let sema = Semantics :: new (db) ; let mut res : FxIndexSet < Runnable > = FxIndexSet :: default () ; let syntax = sema . parse_guess_edition (position . file_id) . syntax () . clone () ; find_related_tests (& sema , & syntax , position , search_scope , & mut res) ; res . into_iter () . sorted_by (cmp_runnables) . collect () }
    };
}

related_tests!()