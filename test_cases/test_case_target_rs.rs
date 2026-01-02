// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_hir/src/target.rs
// Error: expected square brackets
// Problematic line: line 16

use crate::def::DefKind;
use crate::{Item, ItemKind, TraitItem, TraitItemKind, hir};

#[derive(Copy, Clone, PartialEq, Debug, Eq, HashStable_Generic)]
pub enum GenericParamKind {
    Type,
    Lifetime,
