macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! max_insert {
    () => {
        deps!();
        # [test] # [should_panic] fn max_insert () { let mut sv : SmallVec < i32 , 2 > = smallvec ! [0] ; sv . insert (usize :: MAX , 0) ; }
    };
}

max_insert!()