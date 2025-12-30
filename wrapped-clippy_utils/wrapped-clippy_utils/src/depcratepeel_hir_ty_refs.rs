// Generated macro for peel_hir_ty_refs (function)
macro_rules! Depcratepeel_hir_ty_refs {
() => {
// Module: crate
// Provides: {"peel_hir_ty_refs"}
// Dependencies: {}
# [doc = " Peels off all references on the type. Returns the underlying type and the number of references"] # [doc = " removed."] pub fn peel_hir_ty_refs < 'a > (mut ty : & 'a hir :: Ty < 'a >) -> (& 'a hir :: Ty < 'a > , usize) { let mut count = 0 ; loop { match & ty . kind { TyKind :: Ref (_ , ref_ty) => { ty = ref_ty . ty ; count += 1 ; } , _ => break (ty , count) , } } }
};
}
