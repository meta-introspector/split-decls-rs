macro_rules! deps {
    () => {
        Styles!();
    };
}

macro_rules! impl_376 {
    () => {
        deps!();
        impl Default for & '_ Styles { fn default () -> Self { const STYLES : Styles = Styles :: styled () ; & STYLES } }
    };
}

impl_376!();