macro_rules! deps {
    () => {
        Error!();
        StdError!();
        Boxed!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        # [cfg (any (feature = "std" , not (anyhow_no_core_error)))] impl Boxed { # [cold] pub fn new (self , error : Box < dyn StdError + Send + Sync >) -> Error { let backtrace = backtrace_if_absent ! (&* error) ; Error :: construct_from_boxed (error , backtrace) } }
    };
}

impl_102!();