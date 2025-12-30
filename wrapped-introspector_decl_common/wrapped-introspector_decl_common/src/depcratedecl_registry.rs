// Generated macro for DECL_REGISTRY (static)
macro_rules! DepcrateDECL_REGISTRY {
() => {
// Module: crate
// Provides: {"DECL_REGISTRY"}
// Dependencies: {}
pub static DECL_REGISTRY : Lazy < Mutex < DeclRegistry > > = Lazy :: new (| | Mutex :: new (DeclRegistry :: default ())) ;
};
}
