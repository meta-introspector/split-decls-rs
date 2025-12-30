// Generated macro for impl_266 (impl)
macro_rules! Depcrate_object_implsimpl_266 {
() => {
// Module: crate::object::impls
// Provides: {"impl_266"}
// Dependencies: {}
impl < 'repo > From < Object < 'repo > > for ObjectDetached { fn from (mut v : Object < 'repo >) -> Self { ObjectDetached { id : v . id , kind : v . kind , data : steal_from_freelist (& mut v . data) , } } }
};
}
