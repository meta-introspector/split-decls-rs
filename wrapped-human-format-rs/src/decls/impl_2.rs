macro_rules! deps {
    () => {
        Scales!();
        Formatter!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl Default for Formatter { fn default () -> Self { Formatter { decimals : 2 , separator : " " . to_owned () , scales : Scales :: new () , forced_units : "" . to_owned () , forced_suffix : "" . to_owned () , } } }
    };
}

impl_2!();