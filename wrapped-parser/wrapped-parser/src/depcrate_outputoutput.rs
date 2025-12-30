// Generated macro for Output (struct)
macro_rules! Depcrate_outputOutput {
() => {
// Module: crate::output
// Provides: {"Output"}
// Dependencies: {}
# [doc = " Output of the parser -- a DFS traversal of a concrete syntax tree."] # [doc = ""] # [doc = " Use the [`Output::iter`] method to iterate over traversal steps and consume"] # [doc = " a syntax tree."] # [doc = ""] # [doc = " In a sense, this is just a sequence of [`SyntaxKind`]-colored parenthesis"] # [doc = " interspersed into the original [`crate::Input`]. The output is fundamentally"] # [doc = " coordinated with the input and `n_input_tokens` refers to the number of"] # [doc = " times [`crate::Input::push`] was called."] # [derive (Default)] pub struct Output { # [doc = " 32-bit encoding of events. If LSB is zero, then that's an index into the"] # [doc = " error vector. Otherwise, it's one of the thee other variants, with data encoded as"] # [doc = ""] # [doc = " ```text"] # [doc = " |16 bit kind|8 bit n_input_tokens|4 bit tag|4 bit leftover|"] # [doc = " ``````"] event : Vec < u32 > , error : Vec < String > , }
};
}
