macro_rules! Quadratic {
    () => {
        # [doc = " A utility to calculate steps for quadratic backoff similar to how it's done in `git`."] pub struct Quadratic < Fn > { multiplier : usize , max_multiplier : usize , exponent : usize , transform : Fn , }
    };
}

Quadratic!();