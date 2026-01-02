// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/os_str/bytes.rs
// Error: expected square brackets
// Problematic line: line 14

use crate::sys_common::{AsInner, FromInner, IntoInner};
use crate::{fmt, mem, str};

#[cfg(test)]
mod tests;

#[derive(Hash)]
