// Generated macro for RowIndex (trait)
macro_rules! Depcrate_rowRowIndex {
() => {
// Module: crate::row
// Provides: {"RowIndex"}
// Dependencies: {}
# [doc = " Representing a way to index into database rows"] # [doc = ""] # [doc = " * Crates using existing backends should use existing implementations of"] # [doc = "   this traits. Diesel provides `RowIndex<usize>` and `RowIndex<&str>` for"] # [doc = "   all built-in backends"] # [doc = ""] # [doc = " * Crates implementing custom backends need to provide `RowIndex<usize>` and"] # [doc = "   `RowIndex<&str>` impls for their [`Row`] type."] # [doc = ""] pub trait RowIndex < I > { # [doc = " Get the numeric index inside the current row for the provided index value"] fn idx (& self , idx : I) -> Option < usize > ; }
};
}
