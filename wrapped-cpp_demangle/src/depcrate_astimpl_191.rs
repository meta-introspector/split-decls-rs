// Generated macro for impl_191 (impl)
macro_rules! Depcrate_astimpl_191 {
() => {
// Module: crate::ast
// Provides: {"impl_191"}
// Dependencies: {}
impl < 'subs , W > Demangle < 'subs , W > for ParametricBuiltinType where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; match * self { Self :: FloatN (n) => write ! (ctx , "_Float{}" , n) , Self :: FloatNx (n) => write ! (ctx , "_Float{}x" , n) , Self :: SignedBitInt (n) => write ! (ctx , "signed _BitInt({})" , n) , Self :: UnsignedBitInt (n) => write ! (ctx , "unsigned _BitInt({})" , n) , Self :: SignedBitIntExpression (ref expr) => { write ! (ctx , "signed _BitInt(") ? ; expr . demangle (ctx , scope) ? ; write ! (ctx , ")") } Self :: UnsignedBitIntExpression (ref expr) => { write ! (ctx , "unsigned _BitInt(") ? ; expr . demangle (ctx , scope) ? ; write ! (ctx , ")") } } } }
};
}
