// Generated macro for impl_248 (impl)
macro_rules! Depcrate_astimpl_248 {
() => {
// Module: crate::ast
// Provides: {"impl_248"}
// Dependencies: {}
impl < 'subs , W > Demangle < 'subs , W > for FunctionParam where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; match self . 2 { None => write ! (ctx , "this") , Some (i) => write ! (ctx , "{{parm#{}}}" , i + 1) , } } }
};
}
