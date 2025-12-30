// Generated macro for impl_93 (impl)
macro_rules! Depcrate_object_convertimpl_93 {
() => {
// Module: crate::object::convert
// Provides: {"impl_93"}
// Dependencies: {}
impl < 'a > From < & 'a tree :: Entry > for tree :: EntryRef < 'a > { fn from (other : & 'a tree :: Entry) -> tree :: EntryRef < 'a > { let tree :: Entry { mode , filename , oid } = other ; tree :: EntryRef { mode : * mode , filename : filename . as_ref () , oid , } } }
};
}
