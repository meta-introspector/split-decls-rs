// Generated macro for impl_205 (impl)
macro_rules! Depcrate_astimpl_205 {
() => {
// Module: crate::ast
// Provides: {"impl_205"}
// Dependencies: {}
impl < 'subs , W > DemangleAsInner < 'subs , W > for FunctionType where W : 'subs + DemangleWrite , { fn demangle_as_inner < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle_as_inner ! (self , ctx , scope) ; if ! self . cv_qualifiers . is_empty () { self . cv_qualifiers . demangle (ctx , scope) ? ; } if let Some (ref rq) = self . ref_qualifier { ctx . ensure_space () ? ; rq . demangle (ctx , scope) ? ; } Ok (()) } fn downcast_to_function_type (& self) -> Option < & FunctionType > { Some (self) } }
};
}
