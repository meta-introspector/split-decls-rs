// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_cranelift/src/debuginfo/unwind.rs
// Error: expected square brackets
// Problematic line: line 13

use super::object::WriteDebugInfo;
use crate::prelude::*;

pub(crate) struct UnwindContext {
    endian: RunTimeEndian,
    frame_table: FrameTable,
    cie_id: Option<CieId>,
