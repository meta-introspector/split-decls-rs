// Generated macro for impl_196 (impl)
macro_rules! Depcrate_handlerimpl_196 {
() => {
// Module: crate::handler
// Provides: {"impl_196"}
// Dependencies: {}
# [doc = " Allow users to use `Box<M>` as a message without having to re-impl `Message`"] impl < M > Message for Box < M > where M : Message , { type Result = M :: Result ; }
};
}
