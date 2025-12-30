// Generated macro for impl_2041 (impl)
macro_rules! Depcrate_compat_executorimpl_2041 {
() => {
// Module: crate::compat::executor
// Provides: {"impl_2041"}
// Dependencies: {}
impl < Ex > Spawn03 for Executor01As03 < Ex > where Ex : Executor01 < Executor01Future > + Clone + Send + 'static , { fn spawn_obj (& self , future : FutureObj < 'static , () >) -> Result < () , SpawnError03 > { let future = future . unit_error () . compat () ; self . executor01 . execute (future) . map_err (| _ | SpawnError03 :: shutdown ()) } }
};
}
