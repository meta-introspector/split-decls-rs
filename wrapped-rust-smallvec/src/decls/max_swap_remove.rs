macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! max_swap_remove {
    () => {
        deps!();
        # [test] # [should_panic] fn max_swap_remove () { let mut sv : SmallVec < i32 , 2 > = smallvec ! [0] ; sv . swap_remove (usize :: MAX) ; }
    };
}

max_swap_remove!();