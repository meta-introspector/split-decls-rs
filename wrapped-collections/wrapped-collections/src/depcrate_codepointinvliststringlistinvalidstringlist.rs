// Generated macro for InvalidStringList (enum)
macro_rules! Depcrate_codepointinvliststringlistInvalidStringList {
() => {
// Module: crate::codepointinvliststringlist
// Provides: {"InvalidStringList"}
// Dependencies: {}
# [doc = " Custom Errors for [`CodePointInversionListAndStringList`]."] # [derive (Display , Debug)] pub enum InvalidStringList { # [doc = " A string in the string list had an invalid length"] # [cfg_attr (feature = "alloc" , displaydoc ("Invalid string length for string: {0}"))] InvalidStringLength (# [cfg (feature = "alloc")] String) , # [doc = " A string in the string list appears more than once"] # [cfg_attr (feature = "alloc" , displaydoc ("String list has duplicate: {0}"))] StringListNotUnique (# [cfg (feature = "alloc")] String) , # [doc = " Two strings in the string list compare to each other opposite of sorted order"] # [cfg_attr (feature = "alloc" , displaydoc ("Strings in string list not in sorted order: ({0}, {1})"))] StringListNotSorted (# [cfg (feature = "alloc")] String , # [cfg (feature = "alloc")] String ,) , }
};
}
