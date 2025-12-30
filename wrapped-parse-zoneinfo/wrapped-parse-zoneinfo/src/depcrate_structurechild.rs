// Generated macro for Child (enum)
macro_rules! Depcrate_structureChild {
() => {
// Module: crate::structure
// Provides: {"Child"}
// Dependencies: {}
# [doc = " A child module that needs to be created."] # [doc = ""] # [doc = " The order here is important for `PartialOrd`: submodules need to be"] # [doc = " created before actual time zones, as directories need to be created"] # [doc = " before the files in them can be written."] # [derive (PartialEq , Eq , PartialOrd , Ord , Debug , Copy , Clone)] pub enum Child < 'table > { # [doc = " A module containing **only** submodules, no time zones."] Submodule (& 'table str) , # [doc = " A module containing **only** the details of a time zone."] TimeZone (& 'table str) , }
};
}
