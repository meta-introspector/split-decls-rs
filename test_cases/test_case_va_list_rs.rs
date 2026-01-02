// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/ffi/va_list.rs
// Error: expected square brackets
// Problematic line: line 6

//! Better known as "varargs".

use crate::ffi::c_void;
#[allow(unused_imports)]
use crate::fmt;
use crate::intrinsics::{va_arg, va_copy, va_end};
use crate::marker::{PhantomData, PhantomInvariantLifetime};
