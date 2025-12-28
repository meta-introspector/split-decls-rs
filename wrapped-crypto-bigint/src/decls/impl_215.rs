macro_rules! deps {
    () => {
        NonZero!();
        Zero!();
    };
}

macro_rules! impl_215 {
    () => {
        deps!();
        # [cfg (feature = "zeroize")] impl < T : zeroize :: Zeroize + Zero > zeroize :: Zeroize for NonZero < T > { fn zeroize (& mut self) { self . 0 . zeroize () ; } }
    };
}

impl_215!()