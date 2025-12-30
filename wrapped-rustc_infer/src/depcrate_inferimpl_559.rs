// Generated macro for impl_559 (impl)
macro_rules! Depcrate_inferimpl_559 {
() => {
// Module: crate::infer
// Provides: {"impl_559"}
// Dependencies: {}
impl fmt :: Display for FixupError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . unresolved { TyOrConstInferVar :: TyInt (_) => write ! (f , "cannot determine the type of this integer; \
                 add a suffix to specify the type explicitly") , TyOrConstInferVar :: TyFloat (_) => write ! (f , "cannot determine the type of this number; \
                 add a suffix to specify the type explicitly") , TyOrConstInferVar :: Ty (_) => write ! (f , "unconstrained type") , TyOrConstInferVar :: Const (_) => write ! (f , "unconstrained const value") , } } }
};
}
