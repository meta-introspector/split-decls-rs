// Generated macro for Regions (struct)
macro_rules! Depcrate_coverageinfo_ffiRegions {
() => {
// Module: crate::coverageinfo::ffi
// Provides: {"Regions"}
// Dependencies: {}
# [doc = " Holds tables of the various region types in one struct."] # [doc = ""] # [doc = " Don't pass this struct across FFI; pass the individual region tables as"] # [doc = " pointer/length pairs instead."] # [doc = ""] # [doc = " Each field name has a `_regions` suffix for improved readability after"] # [doc = " exhaustive destructing, which ensures that all region types are handled."] # [derive (Clone , Debug , Default)] pub (crate) struct Regions { pub (crate) code_regions : Vec < CodeRegion > , pub (crate) expansion_regions : Vec < ExpansionRegion > , pub (crate) branch_regions : Vec < BranchRegion > , }
};
}
