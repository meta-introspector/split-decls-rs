// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_errors/src/annotate_snippet_emitter_writer.rs
// Error: expected square brackets
// Problematic line: line 19

use crate::registry::Registry;
use crate::snippet::Line;
use crate::translation::{Translator, to_fluent_args};
use crate::{
    CodeSuggestion, DiagInner, DiagMessage, Emitter, ErrCode, Level, MultiSpan, Style, Subdiag,
};

