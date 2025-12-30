// Generated macro for OpaqueTypeStorageEntries (struct)
macro_rules! Depcrate_infer_opaque_types_tableOpaqueTypeStorageEntries {
() => {
// Module: crate::infer::opaque_types::table
// Provides: {"OpaqueTypeStorageEntries"}
// Dependencies: {}
# [doc = " The number of entries in the opaque type storage at a given point."] # [doc = ""] # [doc = " Used to check that we haven't added any new opaque types after checking"] # [doc = " the opaque types currently in the storage."] # [derive (Default , Debug , Clone , Copy , PartialEq , Eq)] pub struct OpaqueTypeStorageEntries { opaque_types : usize , duplicate_entries : usize , }
};
}
