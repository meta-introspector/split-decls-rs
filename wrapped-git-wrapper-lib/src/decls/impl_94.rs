macro_rules! deps {
    () => {
        Result!();
        RollupLock!();
        DummyRollupLock!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl DummyRollupLock { pub fn load (_root_dir : & Path) -> Result < RollupLock > { Ok (RollupLock :: new ()) } }
    };
}

impl_94!()