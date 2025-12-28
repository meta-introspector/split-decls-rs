macro_rules! div_rem {
    () => {
        # [inline] fn div_rem (x : usize , denominator : usize) -> (usize , usize) { (x / denominator , x % denominator) }
    };
}

div_rem!();