macro_rules! deps {
    () => {
        FillFastSlots!();
    };
}

macro_rules! macro_137 {
    () => {
        deps!();
        # [cfg (feature = "internal-test-strategies")] t ! (tests_full_slots , crate :: strategy :: test_strategies :: FillFastSlots) ;
    };
}

macro_137!();