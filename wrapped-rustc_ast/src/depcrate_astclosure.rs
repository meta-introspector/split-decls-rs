// Generated macro for Closure (struct)
macro_rules! Depcrate_astClosure {
() => {
// Module: crate::ast
// Provides: {"Closure"}
// Dependencies: {}
# [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct Closure { pub binder : ClosureBinder , pub capture_clause : CaptureBy , pub constness : Const , pub coroutine_kind : Option < CoroutineKind > , pub movability : Movability , pub fn_decl : Box < FnDecl > , pub body : Box < Expr > , # [doc = " The span of the declaration block: 'move |...| -> ...'"] pub fn_decl_span : Span , # [doc = " The span of the argument block `|...|`"] pub fn_arg_span : Span , }
};
}
