macro_rules! deps {
    () => {
        Quadratic!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl Quadratic < fn (usize) -> usize > { # [doc = " Create a new quadratic backoff iterator that backs off in randomized, ever increasing steps."] pub fn default_with_random () -> Self { Quadratic { multiplier : 1 , max_multiplier : 1000 , exponent : 1 , transform : randomize , } } }
    };
}

impl_3!();