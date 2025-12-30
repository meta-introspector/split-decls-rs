// Generated macro for SortCase (enum)
macro_rules! Depcrate_fs_filterSortCase {
() => {
// Module: crate::fs::filter
// Provides: {"SortCase"}
// Dependencies: {}
# [doc = " Whether a field should be sorted case-sensitively or case-insensitively."] # [doc = " This determines which of the `natord` functions to use."] # [doc = ""] # [doc = " I kept on forgetting which one was sensitive and which one was"] # [doc = " insensitive. Would a case-sensitive sort put capital letters first because"] # [doc = " it takes the case of the letters into account, or intermingle them with"] # [doc = " lowercase letters because it takes the difference between the two cases"] # [doc = " into account? I gave up and just named these two variants after the"] # [doc = " effects they have."] # [derive (PartialEq , Eq , Debug , Copy , Clone)] pub enum SortCase { # [doc = " Sort files case-sensitively with uppercase first, with ‘A’ coming"] # [doc = " before ‘a’."] ABCabc , # [doc = " Sort files case-insensitively, with ‘A’ being equal to ‘a’."] AaBbCc , }
};
}
