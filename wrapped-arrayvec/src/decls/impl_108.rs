macro_rules! deps {
    () => {
        CapacityError!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        # [cfg (feature = "std")] # [doc = " Requires `features=\"std\"`."] impl < T : Any > Error for CapacityError < T > { }
    };
}

impl_108!();