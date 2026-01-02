// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_session/src/output.rs
// Error: expected square brackets
// Problematic line: line 12

use crate::config::{self, CrateType, OutFileName, OutputFilenames, OutputType};
use crate::errors::{self, CrateNameEmpty, FileIsNotWriteable, InvalidCharacterInCrateName};

pub fn out_filename(
    sess: &Session,
    crate_type: CrateType,
    outputs: &OutputFilenames,
