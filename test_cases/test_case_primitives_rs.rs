// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/ffi/primitives.rs
// Error: expected square brackets
// Problematic line: line 6

//! This module is intentionally standalone to facilitate parsing when retrieving
//! core C types.

macro_rules! type_alias {
    {
      $Docfile:tt, $Alias:ident = $Real:ty;
      $( $Cfg:tt )*
