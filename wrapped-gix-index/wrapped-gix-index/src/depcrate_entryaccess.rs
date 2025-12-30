// Generated macro for access (module)
macro_rules! Depcrate_entryaccess {
() => {
// Module: crate::entry
// Provides: {"access"}
// Dependencies: {}
mod access { use bstr :: { BStr , ByteSlice } ; use crate :: { entry , Entry , State } ; impl Entry { # [doc = " Return an entry's path, relative to the repository, which is extracted from its owning `state`."] pub fn path < 'a > (& self , state : & 'a State) -> & 'a BStr { state . path_backing [self . path . clone ()] . as_bstr () } # [doc = " Return an entry's path using the given `backing`."] pub fn path_in < 'backing > (& self , backing : & 'backing crate :: PathStorageRef) -> & 'backing BStr { backing [self . path . clone ()] . as_bstr () } # [doc = " Return an entry's stage. See [entry::Stage] for possible values."] pub fn stage (& self) -> entry :: Stage { self . flags . stage () } # [doc = " Return an entry's stage as raw number between 0 and 4."] # [doc = " Possible values are:"] # [doc = ""] # [doc = " * 0 = no conflict,"] # [doc = " * 1 = base,"] # [doc = " * 2 = ours,"] # [doc = " * 3 = theirs"] pub fn stage_raw (& self) -> u32 { self . flags . stage_raw () } } }
};
}
