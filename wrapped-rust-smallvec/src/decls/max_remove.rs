macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! max_remove {
    () => {
        deps!();
        # [test] # [should_panic] fn max_remove () { let mut sv : SmallVec < i32 , 2 > = smallvec ! [0] ; sv . remove (usize :: MAX) ; }
    };
}

max_remove!();