// Generated macro for DebuggingInformationEntry (struct)
macro_rules! Depcrate_write_unitDebuggingInformationEntry {
() => {
// Module: crate::write::unit
// Provides: {"DebuggingInformationEntry"}
// Dependencies: {}
# [doc = " A Debugging Information Entry (DIE)."] # [doc = ""] # [doc = " DIEs have a set of attributes and optionally have children DIEs as well."] # [doc = ""] # [doc = " DIEs form a tree without any cycles. This is enforced by specifying the"] # [doc = " parent when creating a DIE, and disallowing changes of parent."] # [derive (Debug)] pub struct DebuggingInformationEntry { id : UnitEntryId , parent : Option < UnitEntryId > , tag : constants :: DwTag , # [doc = " Whether to emit `DW_AT_sibling`."] sibling : bool , attrs : Vec < Attribute > , children : Vec < UnitEntryId > , }
};
}
