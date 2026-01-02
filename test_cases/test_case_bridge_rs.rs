// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_public_bridge/src/bridge.rs
// Error: expected square brackets
// Problematic line: line 12

use super::context::CompilerCtxt;
use super::{Bridge, Tables};

pub trait Error {
    fn new(msg: String) -> Self;
    fn from_internal<T: Debug>(err: T) -> Self;
}
