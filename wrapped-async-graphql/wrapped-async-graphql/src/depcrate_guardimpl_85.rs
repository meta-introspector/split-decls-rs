// Generated macro for impl_85 (impl)
macro_rules! Depcrate_guardimpl_85 {
() => {
// Module: crate::guard
// Provides: {"impl_85"}
// Dependencies: {}
# [cfg_attr (feature = "boxed-trait" , async_trait :: async_trait)] impl < A : Guard + Send + Sync , B : Guard + Send + Sync > Guard for Or < A , B > { async fn check (& self , ctx : & Context < '_ >) -> Result < () > { if self . 0 . check (ctx) . await . is_ok () { return Ok (()) ; } self . 1 . check (ctx) . await } }
};
}
