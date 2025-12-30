// Generated macro for impl_255 (impl)
macro_rules! Depcrate_astimpl_255 {
() => {
// Module: crate::ast
// Provides: {"impl_255"}
// Dependencies: {}
impl < 'subs , W > Demangle < 'subs , W > for TemplateArg where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; match * self { TemplateArg :: Type (ref ty) => ty . demangle (ctx , scope) , TemplateArg :: Expression (ref expr) => expr . demangle (ctx , scope) , TemplateArg :: SimpleExpression (ref expr) => expr . demangle (ctx , scope) , TemplateArg :: ArgPack (ref args) => { ctx . is_template_argument_pack = true ; let mut need_comma = false ; for arg in & args [..] { if need_comma { write ! (ctx , ", ") ? ; } arg . demangle (ctx , scope) ? ; need_comma = true ; } Ok (()) } } } }
};
}
