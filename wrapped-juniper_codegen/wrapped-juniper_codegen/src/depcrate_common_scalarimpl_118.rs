// Generated macro for impl_118 (impl)
macro_rules! Depcrate_common_scalarimpl_118 {
() => {
// Module: crate::common::scalar
// Provides: {"impl_118"}
// Dependencies: {}
impl Parse for AttrValue { fn parse (input : ParseStream < '_ >) -> syn :: Result < Self > { if input . fork () . parse :: < syn :: WherePredicate > () . is_ok () { let pred = input . parse () . unwrap () ; if let syn :: WherePredicate :: Type (p) = pred { Ok (Self :: Generic (p)) } else { Err (syn :: Error :: new (pred . span () , "only type predicates are allowed here" ,)) } } else { input . parse :: < syn :: Type > () . map (Self :: Concrete) } } }
};
}
