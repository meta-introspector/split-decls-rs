// Generated macro for impl_28 (impl)
macro_rules! Depcrate_core_errorimpl_28 {
() => {
// Module: crate::core::error
// Provides: {"impl_28"}
// Dependencies: {}
impl error :: Error for Error { fn source (& self) -> Option < & (dyn error :: Error + 'static) > { match self . inner { ErrorInner :: Io { ref err , .. } => Some (err) , ErrorInner :: Loop { .. } | ErrorInner :: ThreadpoolBusy => None , } } # [allow (deprecated)] fn description (& self) -> & str { match self . inner { ErrorInner :: Io { ref err , .. } => err . description () , ErrorInner :: Loop { .. } => "file system loop found" , ErrorInner :: ThreadpoolBusy => "thread-pool busy" , } } fn cause (& self) -> Option < & dyn error :: Error > { self . source () } }
};
}
