// Generated macro for impl_277 (impl)
macro_rules! Depcrate_errorimpl_277 {
() => {
// Module: crate::error
// Provides: {"impl_277"}
// Dependencies: {}
impl Error { # [doc = " Creates a new error value from `core::fmt::Arguments`."] # [doc = ""] # [doc = " It is expected to use [`format_args!`](format_args) from"] # [doc = " Rust's standard library (available in `core`) to create a"] # [doc = " `core::fmt::Arguments`."] # [doc = ""] # [doc = " Callers should generally use their own error types. But in some"] # [doc = " circumstances, it can be convenient to manufacture a Jiff error value"] # [doc = " specifically."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use jiff::Error;"] # [doc = ""] # [doc = " let err = Error::from_args(format_args!(\"something failed\"));"] # [doc = " assert_eq!(err.to_string(), \"something failed\");"] # [doc = " ```"] pub fn from_args < 'a > (message : core :: fmt :: Arguments < 'a >) -> Error { Error :: from (ErrorKind :: Adhoc (AdhocError :: from_args (message))) } # [inline (never)] # [cold] fn context_impl (self , consequent : Error) -> Error { # [cfg (feature = "alloc")] { let mut err = consequent ; if err . inner . is_none () { err = err ! ("unknown jiff error") ; } let inner = err . inner . as_mut () . unwrap () ; assert ! (inner . cause . is_none () , "cause of consequence must be `None`") ; Arc :: get_mut (inner) . unwrap () . cause = Some (self) ; err } # [cfg (not (feature = "alloc"))] { consequent } } }
};
}
