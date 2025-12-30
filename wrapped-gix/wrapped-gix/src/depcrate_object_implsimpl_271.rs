// Generated macro for impl_271 (impl)
macro_rules! Depcrate_object_implsimpl_271 {
() => {
// Module: crate::object::impls
// Provides: {"impl_271"}
// Dependencies: {}
impl < 'repo > From < Commit < 'repo > > for Object < 'repo > { fn from (mut v : Commit < 'repo >) -> Self { Object { id : v . id , kind : gix_object :: Kind :: Commit , data : steal_from_freelist (& mut v . data) , repo : v . repo , } } }
};
}
