// Generated macro for impl_1647 (impl)
macro_rules! Depcrate_syncimpl_1647 {
() => {
// Module: crate::sync
// Provides: {"impl_1647"}
// Dependencies: {}
# [stable (feature = "arc_error" , since = "1.52.0")] impl < T : core :: error :: Error + ? Sized > core :: error :: Error for Arc < T > { # [allow (deprecated)] fn cause (& self) -> Option < & dyn core :: error :: Error > { core :: error :: Error :: cause (& * * self) } fn source (& self) -> Option < & (dyn core :: error :: Error + 'static) > { core :: error :: Error :: source (& * * self) } fn provide < 'a > (& 'a self , req : & mut core :: error :: Request < 'a >) { core :: error :: Error :: provide (& * * self , req) ; } }
};
}
