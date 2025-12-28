macro_rules! deps {
    () => {
        ReadBuffer!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        # [cfg (feature = "zeroize")] impl < BS : ArraySize > Drop for ReadBuffer < BS > { fn drop (& mut self) { use zeroize :: Zeroize ; self . buffer . zeroize () ; } }
    };
}

impl_5!();