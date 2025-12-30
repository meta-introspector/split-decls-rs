// Generated macro for impl_49 (impl)
macro_rules! Depcrate_snapshot_entryimpl_49 {
() => {
// Module: crate::snapshot::entry
// Provides: {"impl_49"}
// Dependencies: {}
impl < 'a > From < crate :: Entry < 'a > > for EmailEntry { fn from (crate :: Entry { new_name , new_email , old_name , old_email , } : crate :: Entry < 'a > ,) -> Self { let mut new_name = new_name . map (ToOwned :: to_owned) ; let mut new_email = new_email . map (ToOwned :: to_owned) ; let entries_by_old_name = old_name . map (| name | { vec ! [NameEntry { new_name : new_name . take () , new_email : new_email . take () , old_name : name . into () , }] }) . unwrap_or_default () ; EmailEntry { new_name , new_email , old_email : old_email . into () , entries_by_old_name , } } }
};
}
