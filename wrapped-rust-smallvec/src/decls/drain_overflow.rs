macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! drain_overflow {
    () => {
        deps!();
        # [test] # [should_panic] fn drain_overflow () { let mut v : SmallVec < u8 , 8 > = smallvec ! [0] ; v . drain (..= std :: usize :: MAX) ; }
    };
}

drain_overflow!()