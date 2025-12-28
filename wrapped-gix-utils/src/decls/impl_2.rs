macro_rules! deps {
    () => {
        Quadratic!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl Default for Quadratic < fn (usize) -> usize > { fn default () -> Self { Quadratic { multiplier : 1 , max_multiplier : 1000 , exponent : 1 , transform : std :: convert :: identity , } } }
    };
}

impl_2!()