// Generated macro for impl_80 (impl)
macro_rules! Depcrate_ext_treeimpl_80 {
() => {
// Module: crate::ext::tree
// Provides: {"impl_80"}
// Dependencies: {}
impl < 'a > TreeEntryRefExt < 'a > for gix_object :: tree :: EntryRef < 'a > { fn attach < 'repo > (self , repo : & 'repo crate :: Repository) -> crate :: object :: tree :: EntryRef < 'repo , 'a > { crate :: object :: tree :: EntryRef { inner : self , repo } } }
};
}
