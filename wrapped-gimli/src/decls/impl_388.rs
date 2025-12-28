macro_rules! deps {
    () => {
        UnitIndex!();
        Reader!();
        DebugTuIndex!();
        Result!();
    };
}

macro_rules! impl_388 {
    () => {
        deps!();
        impl < R : Reader > DebugTuIndex < R > { # [doc = " Parse the index header."] pub fn index (self) -> Result < UnitIndex < R > > { UnitIndex :: parse (self . section) } }
    };
}

impl_388!();