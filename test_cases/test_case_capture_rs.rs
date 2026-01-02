// MINIMAL TEST CASE for parsing failure in: ../rust/library/backtrace/src/capture.rs
// Error: expected square brackets
// Problematic line: line 4

#![allow(clippy::from_over_into)]

use crate::PrintFmt;
#[cfg(feature = "serde")]
use crate::resolve;
use crate::{BacktraceFmt, Symbol, SymbolName, resolve_frame, trace};
use core::ffi::c_void;
