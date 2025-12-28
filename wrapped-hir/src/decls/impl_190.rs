macro_rules! deps {
    () => {
        TermSearchConfig!();
    };
}

macro_rules! impl_190 {
    () => {
        deps!();
        impl Default for TermSearchConfig { fn default () -> Self { Self { enable_borrowcheck : true , many_alternatives_threshold : 1 , fuel : 1200 } } }
    };
}

impl_190!()