// Generated macro for StringTable (struct)
macro_rules! Depcrate_stringtableStringTable {
() => {
// Module: crate::stringtable
// Provides: {"StringTable"}
// Dependencies: {}
# [doc = " Read-only version of the string table"] # [derive (Debug)] pub struct StringTable { string_data : Vec < u8 > , index : FxHashMap < StringId , Addr > , }
};
}
