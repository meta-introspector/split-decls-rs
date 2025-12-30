// Generated macro for RelocationTarget (enum)
macro_rules! Depcrate_readRelocationTarget {
() => {
// Module: crate::read
// Provides: {"RelocationTarget"}
// Dependencies: {}
# [doc = " The target referenced by a [`Relocation`]."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] # [non_exhaustive] pub enum RelocationTarget { # [doc = " The target is a symbol."] Symbol (SymbolIndex) , # [doc = " The target is a section."] Section (SectionIndex) , # [doc = " The offset is an absolute address."] Absolute , }
};
}
