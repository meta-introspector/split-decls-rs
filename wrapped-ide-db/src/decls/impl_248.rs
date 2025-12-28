macro_rules! deps {
    () => {
        RootDatabase!();
    };
}

macro_rules! impl_248 {
    () => {
        deps!();
        # [salsa_macros :: db] impl salsa :: Database for RootDatabase { }
    };
}

impl_248!()