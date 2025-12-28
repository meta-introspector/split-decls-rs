macro_rules! deps {
    () => {
        Scales!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl Default for Scales { fn default () -> Self { Scales :: SI () } }
    };
}

impl_5!()