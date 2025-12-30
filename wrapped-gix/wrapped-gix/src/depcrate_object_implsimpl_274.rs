// Generated macro for impl_274 (impl)
macro_rules! Depcrate_object_implsimpl_274 {
() => {
// Module: crate::object::impls
// Provides: {"impl_274"}
// Dependencies: {}
impl < 'repo > TryFrom < Object < 'repo > > for Commit < 'repo > { type Error = Object < 'repo > ; fn try_from (mut value : Object < 'repo >) -> Result < Self , Self :: Error > { let repo = value . repo ; match value . kind { object :: Kind :: Commit => Ok (Commit { id : value . id , repo , data : steal_from_freelist (& mut value . data) , }) , _ => Err (value) , } } }
};
}
