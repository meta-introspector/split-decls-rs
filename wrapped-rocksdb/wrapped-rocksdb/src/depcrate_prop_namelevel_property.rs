// Generated macro for level_property (function)
macro_rules! Depcrate_prop_namelevel_property {
() => {
// Module: crate::prop_name
// Provides: {"level_property"}
// Dependencies: {}
# [doc = " Constructs a property name for an ‘at level’ property."] # [doc = ""] # [doc = " `name` is the infix of the property name (e.g. `\"num-files-at-level\"`) and"] # [doc = " `level` is level to get statistics of. The property name is constructed as"] # [doc = " `\"rocksdb.<name><level>\"`."] # [doc = ""] # [doc = " Expects `name` not to contain any interior nul bytes."] pub (crate) unsafe fn level_property (name : & str , level : usize) -> PropertyName { let bytes = format ! ("rocksdb.{name}{level}\0") . into_bytes () ; unsafe { PropertyName :: from_vec_with_nul_unchecked (bytes) } }
};
}
