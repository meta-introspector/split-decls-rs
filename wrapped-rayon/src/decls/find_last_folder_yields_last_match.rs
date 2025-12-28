macro_rules! deps {
    () => {
        MatchPosition!();
        FindFolder!();
    };
}

macro_rules! find_last_folder_yields_last_match {
    () => {
        deps!();
        # [test] fn find_last_folder_yields_last_match () { let best_found = AtomicUsize :: new (0) ; let f = FindFolder { find_op : & (| & _ : & i32 | -> bool { true }) , boundary : 0 , match_position : MatchPosition :: Rightmost , best_found : & best_found , item : None , } ; let f = f . consume (0_i32) . consume (1_i32) . consume (2_i32) ; assert_eq ! (f . complete () , Some (2_i32)) ; }
    };
}

find_last_folder_yields_last_match!();