// Generated macro for ContextParts (struct)
macro_rules! Depcrate_context_implContextParts {
() => {
// Module: crate::context_impl
// Provides: {"ContextParts"}
// Dependencies: {}
pub struct ContextParts < A > where A : Actor , A :: Context : AsyncContext < A > , { addr : AddressSenderProducer < A > , flags : ContextFlags , wait : SmallVec < [ActorWaitItem < A > ; 2] > , items : SmallVec < [Item < A > ; 3] > , handles : SmallVec < [SpawnHandle ; 2] > , }
};
}
