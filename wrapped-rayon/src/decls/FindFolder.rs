macro_rules! deps {
    () => {
        MatchPosition!();
    };
}

macro_rules! FindFolder {
    () => {
        deps!();
        struct FindFolder < 'p , T , P > { find_op : & 'p P , boundary : usize , match_position : MatchPosition , best_found : & 'p AtomicUsize , item : Option < T > , }
    };
}

FindFolder!()