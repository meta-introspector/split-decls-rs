// Generated macro for impl_83 (impl)
macro_rules! Depcrate_guardimpl_83 {
() => {
// Module: crate::guard
// Provides: {"impl_83"}
// Dependencies: {}
# [cfg_attr (feature = "boxed-trait" , async_trait :: async_trait)] impl < A : Guard + Send + Sync , B : Guard + Send + Sync > Guard for And < A , B > { async fn check (& self , ctx : & Context < '_ >) -> Result < () > { self . 0 . check (ctx) . await ? ; self . 1 . check (ctx) . await } }
};
}
