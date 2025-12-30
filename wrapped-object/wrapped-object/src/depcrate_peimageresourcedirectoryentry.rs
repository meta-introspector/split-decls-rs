// Generated macro for ImageResourceDirectoryEntry (struct)
macro_rules! Depcrate_peImageResourceDirectoryEntry {
() => {
// Module: crate::pe
// Provides: {"ImageResourceDirectoryEntry"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageResourceDirectoryEntry { pub name_or_id : U32 < LE > , pub offset_to_data_or_directory : U32 < LE > , }
};
}
