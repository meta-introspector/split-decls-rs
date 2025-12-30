// Generated macro for impl_154 (impl)
macro_rules! Depcrate_contextimpl_154 {
() => {
// Module: crate::context
// Provides: {"impl_154"}
// Dependencies: {}
impl < A , T > ContextFutureSpawner < A > for T where A : Actor , A :: Context : AsyncContext < A > , T : ActorFuture < A , Output = () > + 'static , { # [inline] fn spawn (self , ctx : & mut A :: Context) { let _ = ctx . spawn (self) ; } # [inline] fn wait (self , ctx : & mut A :: Context) { ctx . wait (self) ; } }
};
}
