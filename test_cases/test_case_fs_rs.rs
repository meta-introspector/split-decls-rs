// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/os/dragonfly/fs.rs
// Error: expected square brackets
// Problematic line: line 4

#![stable(feature = "metadata_ext", since = "1.1.0")]

use crate::fs::Metadata;
#[allow(deprecated)]
use crate::os::dragonfly::raw;
use crate::sys_common::AsInner;

