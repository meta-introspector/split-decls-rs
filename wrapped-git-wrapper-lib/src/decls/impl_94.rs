macro_rules! deps {
    () => {
        RollupLock!();
        DummyRollupLock!();
        Result!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl DummyRollupLock { pub fn load (_root_dir : & Path) -> Result < RollupLock > { Ok (RollupLock :: new ()) } }
    };
}

impl_94!();