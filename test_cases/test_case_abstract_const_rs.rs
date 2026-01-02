// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_middle/src/ty/abstract_const.rs
// Error: expected square brackets
// Problematic line: line 6

use rustc_errors::ErrorGuaranteed;
use rustc_macros::{HashStable, TyDecodable, TyEncodable, TypeVisitable};

use crate::ty::{
    self, Const, EarlyBinder, Ty, TyCtxt, TypeFoldable, TypeFolder, TypeSuperFoldable,
    TypeVisitableExt,
};
