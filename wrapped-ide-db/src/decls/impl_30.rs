macro_rules! deps {
    () => {
        RootDatabase!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        # [salsa_macros :: db] impl salsa :: Database for RootDatabase { }
    };
}

impl_30!()