// Generated macro for impl_268 (impl)
macro_rules! Depcrate_object_implsimpl_268 {
() => {
// Module: crate::object::impls
// Provides: {"impl_268"}
// Dependencies: {}
impl < 'repo > From < Tag < 'repo > > for ObjectDetached { fn from (mut v : Tag < 'repo >) -> Self { ObjectDetached { id : v . id , kind : gix_object :: Kind :: Tag , data : steal_from_freelist (& mut v . data) , } } }
};
}
