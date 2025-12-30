// Generated macro for impl_276 (impl)
macro_rules! Depcrate_object_implsimpl_276 {
() => {
// Module: crate::object::impls
// Provides: {"impl_276"}
// Dependencies: {}
impl < 'repo > TryFrom < Object < 'repo > > for Tree < 'repo > { type Error = Object < 'repo > ; fn try_from (mut value : Object < 'repo >) -> Result < Self , Self :: Error > { let repo = value . repo ; match value . kind { object :: Kind :: Tree => Ok (Tree { id : value . id , repo , data : steal_from_freelist (& mut value . data) , }) , _ => Err (value) , } } }
};
}
