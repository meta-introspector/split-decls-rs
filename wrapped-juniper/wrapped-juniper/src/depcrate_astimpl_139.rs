// Generated macro for impl_139 (impl)
macro_rules! Depcrate_astimpl_139 {
() => {
// Module: crate::ast
// Provides: {"impl_139"}
// Dependencies: {}
impl < N , M > fmt :: Display for Type < N , M > where N : AsRef < str > , M : AsRef < [TypeModifier] > , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . modifier () { Some (TypeModifier :: NonNull) => write ! (f , "{}!" , self . borrow_inner ()) , Some (TypeModifier :: List (..)) => write ! (f , "[{}]" , self . borrow_inner ()) , None => write ! (f , "{}" , self . name . as_ref ()) , } } }
};
}
