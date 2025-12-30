// Generated macro for TableStructureEntry (struct)
macro_rules! Depcrate_structureTableStructureEntry {
() => {
// Module: crate::structure
// Provides: {"TableStructureEntry"}
// Dependencies: {}
# [doc = " An entry returned from a `TableStructure` iterator."] # [derive (PartialEq , Debug)] pub struct TableStructureEntry < 'table > { # [doc = " This entry’s name, which *can* still include slashes."] pub name : & 'table str , # [doc = " A vector of sorted child names, which should have no slashes in."] pub children : Vec < Child < 'table > > , }
};
}
