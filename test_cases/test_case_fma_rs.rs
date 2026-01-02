// MINIMAL TEST CASE for parsing failure in: ../rust/library/compiler-builtins/libm/src/math/arch/x86/fma.rs
// Error: expected square brackets
// Problematic line: line 10

use crate::support::Round;
use crate::support::feature_detect::select_once;

pub fn fma(x: f64, y: f64, z: f64) -> f64 {
    select_once! {
        sig: fn(x: f64, y: f64, z: f64) -> f64,
        init: || {
