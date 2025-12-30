// Generated macro for impl_248 (impl)
macro_rules! Depcrate_progress_utilsimpl_248 {
() => {
// Module: crate::progress::utils
// Provides: {"impl_248"}
// Dependencies: {}
impl < T > NestedProgress for DoOrDiscard < T > where T : NestedProgress , { type SubProgress = DoOrDiscard < T :: SubProgress > ; fn add_child (& mut self , name : impl Into < String >) -> Self :: SubProgress { DoOrDiscard (self . 0 . add_child (name)) } fn add_child_with_id (& mut self , name : impl Into < String > , id : Id) -> Self :: SubProgress { DoOrDiscard (self . 0 . add_child_with_id (name , id)) } }
};
}
