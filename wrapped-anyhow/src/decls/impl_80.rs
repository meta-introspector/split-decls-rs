macro_rules! deps {
    () => {
        Error!();
        StdError!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        # [cfg (any (feature = "std" , not (anyhow_no_core_error)))] impl From < Error > for Box < dyn StdError + 'static > { # [cold] fn from (error : Error) -> Self { error . into_boxed_dyn_error () } }
    };
}

impl_80!();