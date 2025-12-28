macro_rules! deps {
    () => {
        HashMarker!();
        XofFixedWrapper!();
        ExtendableOutput!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl < T : ExtendableOutput + HashMarker , S : ArraySize > HashMarker for XofFixedWrapper < T , S > { }
    };
}

impl_76!()