macro_rules! deps {
    () => {
        CtOutput!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        # [cfg (feature = "zeroize")] impl < T : OutputSizeUser > zeroize :: ZeroizeOnDrop for CtOutput < T > { }
    };
}

impl_14!();