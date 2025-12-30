// Generated macro for impl_79 (impl)
macro_rules! Depcrate_guardimpl_79 {
() => {
// Module: crate::guard
// Provides: {"impl_79"}
// Dependencies: {}
# [cfg_attr (feature = "boxed-trait" , async_trait :: async_trait)] impl < T > Guard for T where T : Fn (& Context < '_ >) -> Result < () > + Send + Sync + 'static , { async fn check (& self , ctx : & Context < '_ >) -> Result < () > { self (ctx) } }
};
}
