// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_errors/src/json.rs
// Error: expected square brackets
// Problematic line: line 29

use termcolor::{ColorSpec, WriteColor};

use crate::diagnostic::IsLint;
use crate::emitter::{
    ColorConfig, Destination, Emitter, HumanEmitter, HumanReadableErrorType, OutputTheme,
    TimingEvent, should_show_source_code,
};
