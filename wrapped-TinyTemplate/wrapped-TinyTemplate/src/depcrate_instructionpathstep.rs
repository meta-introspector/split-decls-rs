// Generated macro for PathStep (enum)
macro_rules! Depcrate_instructionPathStep {
() => {
// Module: crate::instruction
// Provides: {"PathStep"}
// Dependencies: {}
# [doc = " TinyTemplate implements a simple bytecode interpreter for its template engine. Instructions"] # [doc = " for this interpreter are represented by the Instruction enum and typically contain various"] # [doc = " parameters such as the path to context values or name strings."] # [doc = ""] # [doc = " In TinyTemplate, the template string itself is assumed to be statically available (or at least"] # [doc = " longer-lived than the TinyTemplate instance) so paths and instructions simply borrow string"] # [doc = " slices from the template text. These string slices can then be appended directly to the output"] # [doc = " string."] # [doc = " Enum for a step in a path which optionally contains a parsed index."] # [derive (Eq , PartialEq , Debug , Clone)] pub (crate) enum PathStep < 'template > { Name (& 'template str) , Index (& 'template str , usize) , }
};
}
