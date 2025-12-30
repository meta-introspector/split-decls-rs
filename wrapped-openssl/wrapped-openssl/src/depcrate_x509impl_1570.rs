// Generated macro for impl_1570 (impl)
macro_rules! Depcrate_x509impl_1570 {
() => {
// Module: crate::x509
// Provides: {"impl_1570"}
// Dependencies: {}
impl < 'a > Iterator for X509NameEntries < 'a > { type Item = & 'a X509NameEntryRef ; fn next (& mut self) -> Option < & 'a X509NameEntryRef > { unsafe { match self . nid { Some (nid) => { self . loc = ffi :: X509_NAME_get_index_by_NID (self . name . as_ptr () , nid . as_raw () , self . loc) ; if self . loc == - 1 { return None ; } } None => { self . loc += 1 ; if self . loc >= ffi :: X509_NAME_entry_count (self . name . as_ptr ()) { return None ; } } } let entry = ffi :: X509_NAME_get_entry (self . name . as_ptr () , self . loc) ; Some (X509NameEntryRef :: from_const_ptr_opt (entry) . expect ("entry must not be null")) } } }
};
}
