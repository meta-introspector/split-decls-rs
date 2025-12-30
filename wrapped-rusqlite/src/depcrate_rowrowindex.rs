// Generated macro for RowIndex (trait)
macro_rules! Depcrate_rowRowIndex {
() => {
// Module: crate::row
// Provides: {"RowIndex"}
// Dependencies: {}
# [doc = " A trait implemented by types that can index into columns of a row."] # [doc = ""] # [doc = " It is only implemented for `usize` and `&str`."] pub trait RowIndex : sealed :: Sealed { # [doc = " Returns the index of the appropriate column, or `Error` if no such"] # [doc = " column exists."] fn idx (& self , stmt : & Statement < '_ >) -> Result < usize > ; }
};
}
