macro_rules! deps {
    () => {
        ReaderOffset!();
        UnwindContextStorage!();
        StoreOnHeap!();
        ArrayVec!();
    };
}

macro_rules! RegisterRuleMap {
    () => {
        deps!();
        struct RegisterRuleMap < T , S = StoreOnHeap > where T : ReaderOffset , S : UnwindContextStorage < T > , { rules : ArrayVec < S :: Rules > , }
    };
}

RegisterRuleMap!();