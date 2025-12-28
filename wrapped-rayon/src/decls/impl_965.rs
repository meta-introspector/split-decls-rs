macro_rules! deps {
    () => {
        Collector!();
    };
}

macro_rules! impl_965 {
    () => {
        deps!();
        impl < FromT > Default for Collector < FromT > { fn default () -> Self { Collector { result : None } } }
    };
}

impl_965!();