// Generated macro for StringComponent (enum)
macro_rules! Depcrate_stringtableStringComponent {
() => {
// Module: crate::stringtable
// Provides: {"StringComponent"}
// Dependencies: {}
# [doc = " A single component of a string. Used for building composite table entries."] pub enum StringComponent < 's > { Value (& 's str) , Ref (StringId) , }
};
}
