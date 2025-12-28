macro_rules! deps {
    () => {
        SpawnExt!();
    };
}

macro_rules! impl_992 {
    () => {
        deps!();
        impl < Sp : ? Sized > SpawnExt for Sp where Sp : Spawn { }
    };
}

impl_992!();