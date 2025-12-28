macro_rules! deps {
    () => {
        Dfs!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl < N , VM > Default for Dfs < N , VM > where VM : Default , { fn default () -> Self { Dfs { stack : Vec :: new () , discovered : VM :: default () , } } }
    };
}

impl_43!()