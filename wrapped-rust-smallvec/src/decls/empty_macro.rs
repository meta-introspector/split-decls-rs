macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! empty_macro {
    () => {
        deps!();
        # [test] fn empty_macro () { let _v : SmallVec < u8 , 1 > = smallvec ! [] ; }
    };
}

empty_macro!();