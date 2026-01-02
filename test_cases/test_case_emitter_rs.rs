// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_errors/src/emitter.rs
// Error: expected square brackets
// Problematic line: line 32

use tracing::{debug, instrument, trace, warn};

use crate::registry::Registry;
use crate::snippet::{
    Annotation, AnnotationColumn, AnnotationType, Line, MultilineAnnotation, Style, StyledString,
};
use crate::styled_buffer::StyledBuffer;
