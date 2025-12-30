// Generated macro for impl_262 (impl)
macro_rules! Depcrate_astimpl_262 {
() => {
// Module: crate::ast
// Provides: {"impl_262"}
// Dependencies: {}
impl Expression { fn demangle_as_subexpr < 'subs , 'prev , 'ctx , W > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result where W : 'subs + DemangleWrite , { let needs_parens = match * self { Expression :: FunctionParam (_) | Expression :: Primary (ExprPrimary :: External (_)) => false , _ => true , } ; if needs_parens { write ! (ctx , "(") ? ; } self . demangle (ctx , scope) ? ; if needs_parens { write ! (ctx , ")") ? ; } Ok (()) } }
};
}
