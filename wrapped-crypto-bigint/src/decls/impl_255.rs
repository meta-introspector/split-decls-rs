macro_rules! deps {
    () => {
        Odd!();
    };
}

macro_rules! impl_255 {
    () => {
        deps!();
        # [cfg (feature = "zeroize")] impl < T : zeroize :: Zeroize > zeroize :: Zeroize for Odd < T > { fn zeroize (& mut self) { self . 0 . zeroize () ; } }
    };
}

impl_255!();