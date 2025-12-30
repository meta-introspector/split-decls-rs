// Generated macro for ContextFut (struct)
macro_rules! Depcrate_context_implContextFut {
() => {
// Module: crate::context_impl
// Provides: {"ContextFut"}
// Dependencies: {}
pub struct ContextFut < A , C > where C : AsyncContextParts < A > + Unpin , A : Actor < Context = C > , { ctx : C , act : A , mailbox : Mailbox < A > , wait : SmallVec < [ActorWaitItem < A > ; 2] > , items : SmallVec < [Item < A > ; 3] > , }
};
}
