// Generated macro for impl_7615 (impl)
macro_rules! Depcrate_needless_arbitrary_self_typeimpl_7615 {
() => {
// Module: crate::needless_arbitrary_self_type
// Provides: {"impl_7615"}
// Dependencies: {}
impl EarlyLintPass for NeedlessArbitrarySelfType { fn check_param (& mut self , cx : & EarlyContext < '_ > , p : & Param) { if ! p . is_self () || p . span . from_expansion () { return ; } match & p . ty . kind { TyKind :: Path (None , path) => { if let PatKind :: Ident (BindingMode (ByRef :: No , mutbl) , _ , _) = p . pat . kind { check_param_inner (cx , path , p . span . to (p . ty . span) , & Mode :: Value , mutbl) ; } } , TyKind :: Ref (lifetime , mut_ty) => { if let TyKind :: Path (None , path) = & mut_ty . ty . kind && let PatKind :: Ident (BindingMode :: NONE , _ , _) = p . pat . kind { check_param_inner (cx , path , p . span . to (p . ty . span) , & Mode :: Ref (* lifetime) , mut_ty . mutbl) ; } } , _ => { } , } } }
};
}
