macro_rules! deps {
    () => {
        FillFastSlots!();
    };
}

macro_rules! internal_strategies {
    () => {
        deps!();
        # [cfg (all (feature = "internal-test-strategies" , test))] # [allow (deprecated)] mod internal_strategies { use super :: * ; t ! (tests_full_slots , crate :: strategy :: test_strategies :: FillFastSlots) ; }
    };
}

internal_strategies!();