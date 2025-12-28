macro_rules! deps {
    () => {
        TestDB!();
    };
}

macro_rules! impl_588 {
    () => {
        deps!();
        # [salsa_macros :: db] impl salsa :: Database for TestDB { }
    };
}

impl_588!();