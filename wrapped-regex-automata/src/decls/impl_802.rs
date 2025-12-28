macro_rules! deps {
    () => {
        SerializeError!();
    };
}

macro_rules! impl_802 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for SerializeError { }
    };
}

impl_802!()