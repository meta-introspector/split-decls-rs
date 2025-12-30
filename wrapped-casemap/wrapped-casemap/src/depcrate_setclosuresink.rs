// Generated macro for ClosureSink (trait)
macro_rules! Depcrate_setClosureSink {
() => {
// Module: crate::set
// Provides: {"ClosureSink"}
// Dependencies: {}
# [doc = " An object that accepts characters and/or strings"] # [doc = " to be used with [`CaseMapCloserBorrowed::add_string_case_closure_to()`]"] # [doc = " and [`CaseMapCloserBorrowed::add_case_closure_to()`]."] # [doc = ""] # [doc = " Usually this object"] # [doc = " will be some kind of set over codepoints and strings, or something that"] # [doc = " can be built into one."] # [doc = ""] # [doc = " An implementation is provided for [`CodePointInversionListBuilder`], but users are encouraged"] # [doc = " to implement this trait on their own collections as needed."] # [doc = ""] # [doc = " [`CaseMapCloserBorrowed::add_string_case_closure_to()`]: crate::CaseMapCloserBorrowed::add_string_case_closure_to"] # [doc = " [`CaseMapCloserBorrowed::add_case_closure_to()`]: crate::CaseMapCloserBorrowed::add_case_closure_to"] pub trait ClosureSink { # [doc = " Add a character to the set"] fn add_char (& mut self , c : char) ; # [doc = " Add a string to the set"] fn add_string (& mut self , string : & str) ; }
};
}
