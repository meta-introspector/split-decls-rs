// Generated macro for impl_242 (impl)
macro_rules! Depcrate_progress_utilsimpl_242 {
() => {
// Module: crate::progress::utils
// Provides: {"impl_242"}
// Dependencies: {}
impl < L , R > NestedProgress for Either < L , R > where L : NestedProgress , R : NestedProgress , { type SubProgress = Either < L :: SubProgress , R :: SubProgress > ; fn add_child (& mut self , name : impl Into < String >) -> Self :: SubProgress { match self { Either :: Left (l) => Either :: Left (l . add_child (name)) , Either :: Right (r) => Either :: Right (r . add_child (name)) , } } fn add_child_with_id (& mut self , name : impl Into < String > , id : Id) -> Self :: SubProgress { match self { Either :: Left (l) => Either :: Left (l . add_child_with_id (name , id)) , Either :: Right (r) => Either :: Right (r . add_child_with_id (name , id)) , } } }
};
}
