// Generated macro for impl_27 (impl)
macro_rules! Depcrate_errorimpl_27 {
() => {
// Module: crate::error
// Provides: {"impl_27"}
// Dependencies: {}
# [cfg (feature = "use_std")] impl StdError for Error { fn description (& self) -> & str { match description (self . 0 . get () as c_int) { Some (m) => m , None => "Unknown error" , } } fn cause (& self) -> Option < & dyn StdError > { None } fn source (& self) -> Option < & (dyn StdError + 'static) > { None } }
};
}
