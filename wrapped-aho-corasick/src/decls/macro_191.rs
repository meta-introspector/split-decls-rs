macro_rules! deps {
    () => {
        Config!();
    };
}

macro_rules! macro_191 {
    () => {
        deps!();
        testconfig ! (search_default_leftmost_first , PACKED_LEFTMOST_FIRST , | _ : & mut Config | { }) ;
    };
}

macro_191!()