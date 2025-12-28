macro_rules! deps {
    () => {
        MatchPosition!();
        FindFolder!();
    };
}

macro_rules! find_first_folder_does_not_clobber_first_found {
    () => {
        deps!();
        # [test] fn find_first_folder_does_not_clobber_first_found () { let best_found = AtomicUsize :: new (usize :: MAX) ; let f = FindFolder { find_op : & (| & _ : & i32 | -> bool { true }) , boundary : 0 , match_position : MatchPosition :: Leftmost , best_found : & best_found , item : None , } ; let f = f . consume (0_i32) . consume (1_i32) . consume (2_i32) ; assert ! (f . full ()) ; assert_eq ! (f . complete () , Some (0_i32)) ; }
    };
}

find_first_folder_does_not_clobber_first_found!()