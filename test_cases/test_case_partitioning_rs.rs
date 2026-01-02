// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_monomorphize/src/partitioning.rs
// Error: expected square brackets
// Problematic line: line 112

use rustc_middle::bug;
use rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrFlags;
use rustc_middle::middle::exported_symbols::{SymbolExportInfo, SymbolExportLevel};
use rustc_middle::mir::mono::{
    CodegenUnit, CodegenUnitNameBuilder, InstantiationMode, MonoItem, MonoItemData,
    MonoItemPartitions, Visibility,
};
