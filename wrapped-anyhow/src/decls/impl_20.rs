macro_rules! deps {
    () => {
        ChainState!();
        Chain!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        # [cfg (any (feature = "std" , not (anyhow_no_core_error)))] impl Default for Chain < '_ > { fn default () -> Self { Chain { state : ChainState :: Buffered { rest : Vec :: new () . into_iter () , } , } } }
    };
}

impl_20!();