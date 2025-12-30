// Generated macro for get_details_entry (function)
macro_rules! Depcrate_dirget_details_entry {
() => {
// Module: crate::dir
// Provides: {"get_details_entry"}
// Dependencies: {}
# [doc = " Returned information about directory entry with information which you choose in config."] # [doc = ""] # [doc = " This function takes to arguments:"] # [doc = ""] # [doc = " * `path` - Path to directory."] # [doc = ""] # [doc = " * `config` - Set attributes which you want see inside return data."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function will return an error in the following situations, but is not limited to just"] # [doc = " these cases:"] # [doc = ""] # [doc = " * This `path` does not exist."] # [doc = " * Invalid `path`."] # [doc = " * The current process does not have the permission to access `path`."] # [doc = ""] # [doc = " #Examples"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " extern crate fs_extra;"] # [doc = " use fs_extra::dir::{get_details_entry, DirEntryAttr};"] # [doc = " use std::collections::{HashMap, HashSet};"] # [doc = ""] # [doc = " let mut config = HashSet::new();"] # [doc = " config.insert(DirEntryAttr::Name);"] # [doc = " config.insert(DirEntryAttr::Size);"] # [doc = ""] # [doc = " let entry_info = get_details_entry(\"test\", &config);"] # [doc = " assert_eq!(2, entry_info.len());"] # [doc = " ```"] pub fn get_details_entry < P > (path : P , config : & HashSet < DirEntryAttr > ,) -> Result < HashMap < DirEntryAttr , DirEntryValue > > where P : AsRef < Path > , { let path = path . as_ref () ; let metadata = path . metadata () ? ; get_details_entry_with_meta (path , config , metadata) }
};
}
