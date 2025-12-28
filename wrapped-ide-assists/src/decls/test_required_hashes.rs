macro_rules! test_required_hashes {
    () => {
        # [test] fn test_required_hashes () { assert_eq ! (0 , required_hashes ("abc")) ; assert_eq ! (0 , required_hashes ("###")) ; assert_eq ! (1 , required_hashes ("\"")) ; assert_eq ! (2 , required_hashes ("\"#abc")) ; assert_eq ! (0 , required_hashes ("#abc")) ; assert_eq ! (3 , required_hashes ("#ab\"##c")) ; assert_eq ! (5 , required_hashes ("#ab\"##\"####c")) ; }
    };
}

test_required_hashes!();