macro_rules! deps {
    () => {
        ExtendableOutput!();
        XofFixedWrapper!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        # [cfg (feature = "zeroize")] impl < T : ExtendableOutput + zeroize :: ZeroizeOnDrop , S : ArraySize > zeroize :: ZeroizeOnDrop for XofFixedWrapper < T , S > { }
    };
}

impl_89!()