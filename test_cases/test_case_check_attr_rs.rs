// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_passes/src/check_attr.rs
// Error: expected square brackets
// Problematic line: line 17

use rustc_attr_parsing::{AttributeParser, Late};
use rustc_data_structures::fx::FxHashMap;
use rustc_errors::{Applicability, DiagCtxtHandle, IntoDiagArg, MultiSpan, StashKey};
use rustc_feature::{
    ACCEPTED_LANG_FEATURES, AttributeDuplicates, AttributeType, BUILTIN_ATTRIBUTE_MAP,
    BuiltinAttribute,
};
