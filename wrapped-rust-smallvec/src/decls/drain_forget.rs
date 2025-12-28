macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! drain_forget {
    () => {
        deps!();
        # [test] fn drain_forget () { let mut v : SmallVec < u8 , 1 > = smallvec ! [0 , 1 , 2 , 3 , 4 , 5 , 6 , 7] ; std :: mem :: forget (v . drain (2 .. 5)) ; assert_eq ! (v . len () , 2) ; }
    };
}

drain_forget!()