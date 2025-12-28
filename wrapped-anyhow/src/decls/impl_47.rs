macro_rules! deps {
    () => {
        Error!();
        StdError!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        # [cfg (any (feature = "std" , not (anyhow_no_core_error)))] impl < E > From < E > for Error where E : StdError + Send + Sync + 'static , { # [cold] fn from (error : E) -> Self { let backtrace = backtrace_if_absent ! (& error) ; Error :: construct_from_std (error , backtrace) } }
    };
}

impl_47!();