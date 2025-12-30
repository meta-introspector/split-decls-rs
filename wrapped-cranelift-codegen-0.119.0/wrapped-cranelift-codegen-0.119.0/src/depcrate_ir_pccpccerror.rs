// Generated macro for PccError (enum)
macro_rules! Depcrate_ir_pccPccError {
() => {
// Module: crate::ir::pcc
// Provides: {"PccError"}
// Dependencies: {}
# [doc = " An error or inconsistency discovered when checking proof-carrying"] # [doc = " code."] # [derive (Debug , Clone)] pub enum PccError { # [doc = " An operation wraps around, invalidating the stated value"] # [doc = " range."] Overflow , # [doc = " An input to an operator that produces a fact-annotated value"] # [doc = " does not have a fact describing it, and one is needed."] MissingFact , # [doc = " A derivation of an output fact is unsupported (incorrect or"] # [doc = " not derivable)."] UnsupportedFact , # [doc = " A block parameter claims a fact that one of its predecessors"] # [doc = " does not support."] UnsupportedBlockparam , # [doc = " A memory access is out of bounds."] OutOfBounds , # [doc = " Proof-carrying-code checking is not implemented for a"] # [doc = " particular compiler backend."] UnimplementedBackend , # [doc = " Proof-carrying-code checking is not implemented for a"] # [doc = " particular instruction that instruction-selection chose. This"] # [doc = " is an internal compiler error."] UnimplementedInst , # [doc = " Access to an invalid or undefined field offset in a struct."] InvalidFieldOffset , # [doc = " Access to a field via the wrong type."] BadFieldType , # [doc = " Store to a read-only field."] WriteToReadOnlyField , # [doc = " Store of data to a field with a fact that does not subsume the"] # [doc = " field's fact."] InvalidStoredFact , }
};
}
