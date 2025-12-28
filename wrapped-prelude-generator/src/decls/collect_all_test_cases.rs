macro_rules! deps {
    () => {
        TestInfo!();
    };
}

macro_rules! collect_all_test_cases {
    () => {
        deps!();
        # [doc = " Collects all unique test functions from the repository."] pub fn collect_all_test_cases (repo_root : & Path) -> Result < Vec < TestInfo > > { let mut all_test_functions = Vec :: new () ; let mut seen_test_names = HashSet :: new () ; for entry in WalkDir :: new (repo_root) . into_iter () . filter_map (| e | e . ok ()) . filter (| e | { e . file_type () . is_file () && e . path () . extension () . map_or (false , | ext | ext == "rs") }) { let file_path = entry . path () ; println ! ("  -> Processing file for tests: {}" , file_path . display ()) ; match extract_test_cases_from_file (file_path) { Ok (functions) => { for func_info in functions { if seen_test_names . insert (func_info . name . clone ()) { all_test_functions . push (func_info) ; } } } Err (e) => { eprintln ! ("Warning: Could not extract tests from {}: {}" , file_path . display () , e) ; } } } Ok (all_test_functions) }
    };
}

collect_all_test_cases!();