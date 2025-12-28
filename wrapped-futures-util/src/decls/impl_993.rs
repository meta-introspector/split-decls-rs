macro_rules! deps {
    () => {
        LocalSpawnExt!();
    };
}

macro_rules! impl_993 {
    () => {
        deps!();
        impl < Sp : ? Sized > LocalSpawnExt for Sp where Sp : LocalSpawn { }
    };
}

impl_993!();