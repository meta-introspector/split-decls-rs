macro_rules! deps {
    () => {
        OutputReader!();
    };
}

macro_rules! impl_213 {
    () => {
        deps!();
        # [cfg (feature = "zeroize")] impl Zeroize for OutputReader { fn zeroize (& mut self) { let Self { inner , position_within_block , } = self ; inner . zeroize () ; position_within_block . zeroize () ; } }
    };
}

impl_213!();