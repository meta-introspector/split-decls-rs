// Generated macro for TemplateCompiler (struct)
macro_rules! Depcrate_compilerTemplateCompiler {
() => {
// Module: crate::compiler
// Provides: {"TemplateCompiler"}
// Dependencies: {}
# [doc = " The TemplateCompiler struct is responsible for parsing a template string and generating bytecode"] # [doc = " instructions based on it. The parser is a simple hand-written pattern-matching parser with no"] # [doc = " recursion, which makes it relatively easy to read."] pub (crate) struct TemplateCompiler < 'template > { original_text : & 'template str , remaining_text : & 'template str , instructions : Vec < Instruction < 'template > > , block_stack : Vec < (& 'template str , Block) > , # [doc = " When we see a `{foo -}` or similar, we need to remember to left-trim the next text block we"] # [doc = " encounter."] trim_next : bool , }
};
}
