macro_rules! deps {
    () => {
        CacheError!();
    };
}

macro_rules! impl_263 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for CacheError { }
    };
}

impl_263!()