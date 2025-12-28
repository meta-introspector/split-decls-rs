macro_rules! deps {
    () => {
        GraphiQLVersion!();
    };
}

macro_rules! impl_618 {
    () => {
        deps!();
        impl Default for GraphiQLVersion < '_ > { fn default () -> Self { Self ("4") } }
    };
}

impl_618!();