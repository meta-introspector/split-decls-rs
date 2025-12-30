// Generated macro for impl_217 (impl)
macro_rules! Depcrate_astimpl_217 {
() => {
// Module: crate::ast
// Provides: {"impl_217"}
// Dependencies: {}
impl < 'subs , W > Demangle < 'subs , W > for ClassEnumType where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; match * self { ClassEnumType :: Named (ref name) => name . demangle (ctx , scope) , ClassEnumType :: ElaboratedStruct (ref name) => { write ! (ctx , "class ") ? ; name . demangle (ctx , scope) } ClassEnumType :: ElaboratedUnion (ref name) => { write ! (ctx , "union ") ? ; name . demangle (ctx , scope) } ClassEnumType :: ElaboratedEnum (ref name) => { write ! (ctx , "enum ") ? ; name . demangle (ctx , scope) } } } }
};
}
