mkuse!{use rustc_macros :: Diagnostic ;}
mkuse!{use rustc_span :: Span ;}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (infer_opaque_hidden_type)] pub (crate) struct OpaqueHiddenTypeDiag { # [primary_span] # [label] pub span : Span , # [note (infer_opaque_type)] pub opaque_type : Span , # [note (infer_hidden_type)] pub hidden_type : Span , }}}