// Generated macro for impl_211 (impl)
macro_rules! Depcrate_astimpl_211 {
() => {
// Module: crate::ast
// Provides: {"impl_211"}
// Dependencies: {}
impl < 'subs , W > DemangleAsInner < 'subs , W > for BareFunctionType where W : 'subs + DemangleWrite , { fn demangle_as_inner < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle_as_inner ! (self , ctx , scope) ; self . args () . demangle_as_inner (ctx , scope) ? ; Ok (()) } }
};
}
