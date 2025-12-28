macro_rules! deps {
    () => {
        TestItem!();
    };
}

macro_rules! discover_tests_in_file {
    () => {
        deps!();
        pub (crate) fn discover_tests_in_file (db : & RootDatabase , file_id : FileId) -> Vec < TestItem > { let sema = Semantics :: new (db) ; let Some (module) = sema . file_to_module_def (file_id) else { return vec ! [] } ; let Some ((mut tests , id)) = find_module_id_and_test_parents (& sema , module) else { return vec ! [] ; } ; tests . extend (discover_tests_in_module (db , module , id , true)) ; tests }
    };
}

discover_tests_in_file!();