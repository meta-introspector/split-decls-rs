macro_rules! deps {
    () => {
        Hash!();
    };
}

macro_rules! impl_173 {
    () => {
        deps!();
        # [cfg (feature = "zeroize")] impl Zeroize for Hash { fn zeroize (& mut self) { let Self (bytes) = self ; bytes . zeroize () ; } }
    };
}

impl_173!()