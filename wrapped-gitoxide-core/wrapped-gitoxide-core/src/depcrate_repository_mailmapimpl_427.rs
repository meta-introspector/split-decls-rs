// Generated macro for impl_427 (impl)
macro_rules! Depcrate_repository_mailmapimpl_427 {
() => {
// Module: crate::repository::mailmap
// Provides: {"impl_427"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'a > From < Entry < 'a > > for JsonEntry { fn from (v : Entry < 'a >) -> Self { use gix :: bstr :: ByteSlice ; JsonEntry { new_name : v . new_name () . map (| s | s . to_str_lossy () . into_owned ()) , new_email : v . new_email () . map (| s | s . to_str_lossy () . into_owned ()) , old_name : v . old_name () . map (| s | s . to_str_lossy () . into_owned ()) , old_email : v . old_email () . to_str_lossy () . into_owned () , } } }
};
}
