macro_rules! deps {
    () => {
        ReaderOffset!();
        ArrayVec!();
        StoreOnHeap!();
        UnwindContextStorage!();
    };
}

macro_rules! RegisterRuleMap {
    () => {
        deps!();
        struct RegisterRuleMap < T , S = StoreOnHeap > where T : ReaderOffset , S : UnwindContextStorage < T > , { rules : ArrayVec < S :: Rules > , }
    };
}

RegisterRuleMap!()