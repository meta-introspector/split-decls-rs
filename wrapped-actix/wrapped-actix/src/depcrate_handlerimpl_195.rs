// Generated macro for impl_195 (impl)
macro_rules! Depcrate_handlerimpl_195 {
() => {
// Module: crate::handler
// Provides: {"impl_195"}
// Dependencies: {}
# [doc = " Allow users to use `Arc<M>` as a message without having to re-impl `Message`"] impl < M > Message for Arc < M > where M : Message , { type Result = M :: Result ; }
};
}
