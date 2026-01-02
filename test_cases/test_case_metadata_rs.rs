// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_ssa/src/back/metadata.rs
// Error: expected square brackets
// Problematic line: line 10


use itertools::Itertools;
use object::write::{self, StandardSegment, Symbol, SymbolSection};
use object::{
    Architecture, BinaryFormat, Endianness, FileFlags, Object, ObjectSection, ObjectSymbol,
    SectionFlags, SectionKind, SymbolFlags, SymbolKind, SymbolScope, elf, pe, xcoff,
};
