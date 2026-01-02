// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_cranelift/src/debuginfo/line_info.rs
// Error: expected square brackets
// Problematic line: line 9

use cranelift_codegen::MachSrcLoc;
use cranelift_codegen::binemit::CodeOffset;
use gimli::write::{AttributeValue, FileId, FileInfo, LineProgram, LineString, LineStringTable};
use rustc_span::{
    FileName, Pos, SourceFile, SourceFileAndLine, SourceFileHash, SourceFileHashAlgorithm, hygiene,
};

