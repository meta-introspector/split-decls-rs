macro_rules! deps {
    () => {
        IoRead!();
        Read!();
    };
}

macro_rules! impl_576 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl < R > private :: Sealed for IoRead < R > where R : io :: Read { }
    };
}

impl_576!()