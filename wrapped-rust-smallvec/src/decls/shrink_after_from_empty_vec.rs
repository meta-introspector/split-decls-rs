macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! shrink_after_from_empty_vec {
    () => {
        deps!();
        # [test] fn shrink_after_from_empty_vec () { let mut v = SmallVec :: < u8 , 2 > :: from_vec (vec ! []) ; v . shrink_to_fit () ; assert ! (! v . spilled ()) }
    };
}

shrink_after_from_empty_vec!();