macro_rules! deps {
    () => {
        Output!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        # [cfg (feature = "zeroize")] impl Zeroize for Output { fn zeroize (& mut self) { let Self { input_chaining_value , block , block_len , counter , flags , platform : _ , } = self ; input_chaining_value . zeroize () ; block . zeroize () ; block_len . zeroize () ; counter . zeroize () ; flags . zeroize () ; } }
    };
}

impl_52!()