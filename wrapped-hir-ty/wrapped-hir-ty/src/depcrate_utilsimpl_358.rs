// Generated macro for impl_358 (impl)
macro_rules! Depcrate_utilsimpl_358 {
() => {
// Module: crate::utils
// Provides: {"impl_358"}
// Dependencies: {}
impl FallibleTypeFolder < Interner > for UnevaluatedConstEvaluatorFolder < '_ > { type Error = () ; fn as_dyn (& mut self) -> & mut dyn FallibleTypeFolder < Interner , Error = () > { self } fn interner (& self) -> Interner { Interner } fn try_fold_const (& mut self , constant : Const , _outer_binder : DebruijnIndex ,) -> Result < Const , Self :: Error > { if let chalk_ir :: ConstValue :: Concrete (c) = & constant . data (Interner) . value && let ConstScalar :: UnevaluatedConst (id , subst) = & c . interned { if let Ok (eval) = self . db . const_eval (* id , subst . clone () , None) { return Ok (eval) ; } else { return Ok (unknown_const (constant . data (Interner) . ty . clone ())) ; } } Ok (constant) } }
};
}
