mkuse!{use rustc_macros :: Diagnostic ;}
mkuse!{use rustc_span :: Span ;}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (codegen_gcc_unwinding_inline_asm)] pub (crate) struct UnwindingInlineAsm { # [primary_span] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (codegen_gcc_copy_bitcode)] pub (crate) struct CopyBitcode { pub err : std :: io :: Error , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (codegen_gcc_lto_bitcode_from_rlib)] pub (crate) struct LtoBitcodeFromRlib { pub gcc_err : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (codegen_gcc_explicit_tail_calls_unsupported)] pub (crate) struct ExplicitTailCallsUnsupported ;}}