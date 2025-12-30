// Generated macro for write_clif_file (function)
macro_rules! Depcrate_pretty_clifwrite_clif_file {
() => {
// Module: crate::pretty_clif
// Provides: {"write_clif_file"}
// Dependencies: {}
pub (crate) fn write_clif_file (output_filenames : & OutputFilenames , symbol_name : & str , postfix : & str , isa : & dyn cranelift_codegen :: isa :: TargetIsa , func : & cranelift_codegen :: ir :: Function , mut clif_comments : & CommentWriter ,) { write_ir_file (output_filenames , & format ! ("{}.{}.clif" , symbol_name , postfix) , | file | { let mut clif = String :: new () ; cranelift_codegen :: write :: decorate_function (& mut clif_comments , & mut clif , func) . unwrap () ; for flag in isa . flags () . iter () { writeln ! (file , "set {}" , flag) ? ; } write ! (file , "target {}" , isa . triple () . architecture) ? ; for isa_flag in isa . isa_flags () . iter () { write ! (file , " {}" , isa_flag) ? ; } writeln ! (file , "\n") ? ; writeln ! (file) ? ; file . write_all (clif . as_bytes ()) ? ; Ok (()) }) ; }
};
}
