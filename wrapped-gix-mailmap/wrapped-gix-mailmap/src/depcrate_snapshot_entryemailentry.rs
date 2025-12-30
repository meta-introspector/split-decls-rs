// Generated macro for EmailEntry (struct)
macro_rules! Depcrate_snapshot_entryEmailEntry {
() => {
// Module: crate::snapshot::entry
// Provides: {"EmailEntry"}
// Dependencies: {}
# [derive (Clone , Debug , Eq , PartialEq)] pub (crate) struct EmailEntry { pub (crate) new_name : Option < BString > , pub (crate) new_email : Option < BString > , pub (crate) old_email : EncodedString , pub (crate) entries_by_old_name : Vec < NameEntry > , }
};
}
