// Generated macro for impl_277 (impl)
macro_rules! Depcrate_object_implsimpl_277 {
() => {
// Module: crate::object::impls
// Provides: {"impl_277"}
// Dependencies: {}
impl < 'repo > TryFrom < Object < 'repo > > for Blob < 'repo > { type Error = Object < 'repo > ; fn try_from (mut value : Object < 'repo >) -> Result < Self , Self :: Error > { let repo = value . repo ; match value . kind { object :: Kind :: Blob => Ok (Blob { id : value . id , repo , data : steal_from_freelist (& mut value . data) , }) , _ => Err (value) , } } }
};
}
