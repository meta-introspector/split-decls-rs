macro_rules! deps {
    () => {
        Configuration!();
    };
}

macro_rules! impl_251 {
    () => {
        deps!();
        impl < E , I , L > Default for Configuration < E , I , L > { fn default () -> Self { generate () } }
    };
}

impl_251!();