macro_rules! deps {
    () => {
        SpawnError!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for SpawnError { }
    };
}

impl_7!();