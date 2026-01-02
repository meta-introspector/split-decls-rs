// MINIMAL TEST CASE for parsing failure in: ../rust/library/stdarch/crates/core_arch/src/x86/avx.rs
// Error: expected square brackets
// Problematic line: line 16

//! [amd64_ref]: http://support.amd.com/TechDocs/24594.pdf
//! [wiki]: https://en.wikipedia.org/wiki/Advanced_Vector_Extensions

use crate::{
    core_arch::{simd::*, x86::*},
    intrinsics::simd::*,
    mem, ptr,
