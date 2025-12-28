macro_rules! deps {
    () => {
        DeserializeError!();
    };
}

macro_rules! impl_806 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for DeserializeError { }
    };
}

impl_806!()