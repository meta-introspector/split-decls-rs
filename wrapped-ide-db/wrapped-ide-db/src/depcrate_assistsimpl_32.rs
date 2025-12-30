// Generated macro for impl_32 (impl)
macro_rules! Depcrate_assistsimpl_32 {
() => {
// Module: crate::assists
// Provides: {"impl_32"}
// Dependencies: {}
impl AssistId { pub fn quick_fix (id : & 'static str) -> AssistId { AssistId (id , AssistKind :: QuickFix , None) } pub fn generate (id : & 'static str) -> AssistId { AssistId (id , AssistKind :: Generate , None) } pub fn refactor (id : & 'static str) -> AssistId { AssistId (id , AssistKind :: Refactor , None) } pub fn refactor_extract (id : & 'static str) -> AssistId { AssistId (id , AssistKind :: RefactorExtract , None) } pub fn refactor_inline (id : & 'static str) -> AssistId { AssistId (id , AssistKind :: RefactorInline , None) } pub fn refactor_rewrite (id : & 'static str) -> AssistId { AssistId (id , AssistKind :: RefactorRewrite , None) } }
};
}
