macro_rules! deps {
    () => {
        StdError!();
        Error!();
    };
}

macro_rules! ext {
    () => {
        deps!();
        mod ext { use super :: * ; pub trait StdError { fn ext_context < C > (self , context : C) -> Error where C : Display + Send + Sync + 'static ; } # [cfg (any (feature = "std" , not (anyhow_no_core_error)))] impl < E > StdError for E where E : crate :: StdError + Send + Sync + 'static , { fn ext_context < C > (self , context : C) -> Error where C : Display + Send + Sync + 'static , { let backtrace = backtrace_if_absent ! (& self) ; Error :: construct_from_context (context , self , backtrace) } } impl StdError for Error { fn ext_context < C > (self , context : C) -> Error where C : Display + Send + Sync + 'static , { self . context (context) } } }
    };
}

ext!()