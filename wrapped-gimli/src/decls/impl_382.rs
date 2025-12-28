macro_rules! deps {
    () => {
        Reader!();
        DebugCuIndex!();
        Result!();
        UnitIndex!();
    };
}

macro_rules! impl_382 {
    () => {
        deps!();
        impl < R : Reader > DebugCuIndex < R > { # [doc = " Parse the index header."] pub fn index (self) -> Result < UnitIndex < R > > { UnitIndex :: parse (self . section) } }
    };
}

impl_382!()