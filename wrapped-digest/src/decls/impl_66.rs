macro_rules! deps {
    () => {
        CtOutput!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl < T : OutputSizeUser > Drop for CtOutput < T > { # [inline] fn drop (& mut self) { # [cfg (feature = "zeroize")] { use zeroize :: Zeroize ; self . bytes . zeroize () } } }
    };
}

impl_66!();