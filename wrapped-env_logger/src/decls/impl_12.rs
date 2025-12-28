macro_rules! deps {
    () => {
        Env!();
        Var!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl Default for Env < '_ > { fn default () -> Self { Env { filter : Var :: new (DEFAULT_FILTER_ENV) , write_style : Var :: new (DEFAULT_WRITE_STYLE_ENV) , } } }
    };
}

impl_12!()