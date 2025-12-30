// Generated macro for impl_88 (impl)
macro_rules! Depcrate_arcimpl_88 {
() => {
// Module: crate::arc
// Provides: {"impl_88"}
// Dependencies: {}
# [cfg (any (not (portable_atomic_no_error_in_core) , feature = "std"))] impl < T : ? Sized + error :: Error > error :: Error for Arc < T > { # [allow (deprecated)] fn description (& self) -> & str { error :: Error :: description (& * * self) } # [allow (deprecated)] fn cause (& self) -> Option < & dyn error :: Error > { error :: Error :: cause (& * * self) } fn source (& self) -> Option < & (dyn error :: Error + 'static) > { error :: Error :: source (& * * self) } }
};
}
