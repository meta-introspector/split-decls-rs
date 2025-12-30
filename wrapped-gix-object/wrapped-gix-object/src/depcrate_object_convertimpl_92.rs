// Generated macro for impl_92 (impl)
macro_rules! Depcrate_object_convertimpl_92 {
() => {
// Module: crate::object::convert
// Provides: {"impl_92"}
// Dependencies: {}
impl From < tree :: EntryRef < '_ > > for tree :: Entry { fn from (other : tree :: EntryRef < '_ >) -> tree :: Entry { let tree :: EntryRef { mode , filename , oid } = other ; tree :: Entry { mode , filename : filename . to_owned () , oid : oid . into () , } } }
};
}
