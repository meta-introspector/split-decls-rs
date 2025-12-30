// Generated macro for impl_55 (impl)
macro_rules! Depcrate_astimpl_55 {
() => {
// Module: crate::ast
// Provides: {"impl_55"}
// Dependencies: {}
impl fmt :: Display for ParamKindOrd { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { ParamKindOrd :: Lifetime => "lifetime" . fmt (f) , ParamKindOrd :: TypeOrConst => "type and const" . fmt (f) , } } }
};
}
