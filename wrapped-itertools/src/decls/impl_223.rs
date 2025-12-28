macro_rules! deps {
    () => {
        ExactlyOneError!();
    };
}

macro_rules! impl_223 {
    () => {
        deps!();
        # [cfg (feature = "use_std")] impl < I > Error for ExactlyOneError < I > where I : Iterator + Debug , I :: Item : Debug , { }
    };
}

impl_223!();