// Generated macro for Compiler (struct)
macro_rules! Depcrate_compileCompiler {
() => {
// Module: crate::compile
// Provides: {"Compiler"}
// Dependencies: {}
# [doc = " A compiler translates a regular expression AST to a sequence of"] # [doc = " instructions. The sequence of instructions represents an NFA."] pub struct Compiler { insts : Vec < MaybeInst > , compiled : Program , capture_name_idx : HashMap < String , usize > , num_exprs : usize , size_limit : usize , suffix_cache : SuffixCache , utf8_seqs : Option < Utf8Sequences > , byte_classes : ByteClassSet , }
};
}
