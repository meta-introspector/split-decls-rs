// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/personality/dwarf/eh.rs
// Error: expected square brackets
// Problematic line: line 39


pub const DW_EH_PE_indirect: u8 = 0x80;

#[derive(Copy, Clone)]
pub struct EHContext<'a> {
    pub ip: *const u8,                             // Current instruction pointer
    pub func_start: *const u8,                     // Pointer to the current function
