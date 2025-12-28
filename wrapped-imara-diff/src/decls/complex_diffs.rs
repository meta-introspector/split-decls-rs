macro_rules! deps {
    () => {
        Diff!();
        Algorithm!();
        InternedInput!();
    };
}

macro_rules! complex_diffs {
    () => {
        deps!();
        # [test] # [cfg (not (miri))] fn complex_diffs () { for algorithm in Algorithm :: ALL { println ! ("{algorithm:?}") ; let test_dir = project_root () . join ("tests") ; for (file1 , file2) in [("test1.json" , "test2.json") , ("helix_syntax.rs.Histogram.diff" , "helix_syntax.rs.after") ,] { let path_before = test_dir . join (file1) ; let path_diff = test_dir . join (file2) ; let before = read_to_string (path_before) . unwrap () ; let after = read_to_string (path_diff) . unwrap () ; let input = InternedInput :: new (& * before , & * after) ; let mut diff = Diff :: compute (algorithm , & input) ; println ! ("start postprocess {file1}") ; diff . postprocess_lines (& input) ; println ! ("-{} +{}" , diff . count_removals () , diff . count_additions ()) } } }
    };
}

complex_diffs!();