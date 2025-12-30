// Generated macro for impl_275 (impl)
macro_rules! Depcrate_object_implsimpl_275 {
() => {
// Module: crate::object::impls
// Provides: {"impl_275"}
// Dependencies: {}
impl < 'repo > TryFrom < Object < 'repo > > for Tag < 'repo > { type Error = Object < 'repo > ; fn try_from (mut value : Object < 'repo >) -> Result < Self , Self :: Error > { let repo = value . repo ; match value . kind { object :: Kind :: Tag => Ok (Tag { id : value . id , repo , data : steal_from_freelist (& mut value . data) , }) , _ => Err (value) , } } }
};
}
