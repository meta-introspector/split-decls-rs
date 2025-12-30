// Generated macro for Slice (trait)
macro_rules! Depcrate_parser_sliceSlice {
() => {
// Module: crate::parser::slice
// Provides: {"Slice"}
// Dependencies: {}
pub trait Slice < 's > : AsRef < str > + Clone + PartialEq { fn slice (& self , range : Range < usize >) -> Self ; fn trim (& mut self) ; }
};
}
