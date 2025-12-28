macro_rules! deps {
    () => {
        MacMarker!();
        XofFixedWrapper!();
        ExtendableOutput!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        # [cfg (feature = "mac")] impl < T : ExtendableOutput + crate :: MacMarker , S : ArraySize > crate :: MacMarker for XofFixedWrapper < T , S > { }
    };
}

impl_77!();