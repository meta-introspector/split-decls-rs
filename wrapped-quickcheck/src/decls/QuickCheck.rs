macro_rules! deps {
    () => {
        Gen!();
    };
}

macro_rules! QuickCheck {
    () => {
        deps!();
        # [doc = " The main `QuickCheck` type for setting configuration and running"] # [doc = " `QuickCheck`."] pub struct QuickCheck { tests : u64 , max_tests : u64 , min_tests_passed : u64 , rng : Gen , }
    };
}

QuickCheck!()