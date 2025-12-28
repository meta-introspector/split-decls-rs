macro_rules! deps {
    () => {
        LazyStateIDError!();
    };
}

macro_rules! impl_270 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for LazyStateIDError { }
    };
}

impl_270!()