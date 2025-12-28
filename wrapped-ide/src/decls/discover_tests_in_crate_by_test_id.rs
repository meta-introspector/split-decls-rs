macro_rules! deps {
    () => {
        TestItem!();
    };
}

macro_rules! discover_tests_in_crate_by_test_id {
    () => {
        deps!();
        pub (crate) fn discover_tests_in_crate_by_test_id (db : & RootDatabase , crate_test_id : & str ,) -> Vec < TestItem > { let Some (crate_id) = find_crate_by_id (db , crate_test_id) else { return vec ! [] ; } ; discover_tests_in_crate (db , crate_id) }
    };
}

discover_tests_in_crate_by_test_id!();