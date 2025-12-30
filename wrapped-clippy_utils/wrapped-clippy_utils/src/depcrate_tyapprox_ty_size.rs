// Generated macro for approx_ty_size (function)
macro_rules! Depcrate_tyapprox_ty_size {
() => {
// Module: crate::ty
// Provides: {"approx_ty_size"}
// Dependencies: {}
# [doc = " Comes up with an \"at least\" guesstimate for the type's size, not taking into"] # [doc = " account the layout of type parameters."] pub fn approx_ty_size < 'tcx > (cx : & LateContext < 'tcx > , ty : Ty < 'tcx >) -> u64 { use rustc_middle :: ty :: layout :: LayoutOf ; match (cx . layout_of (ty) . map (| layout | layout . size . bytes ()) , ty . kind ()) { (Ok (size) , _) => size , (Err (_) , ty :: Tuple (list)) => list . iter () . map (| t | approx_ty_size (cx , t)) . sum () , (Err (_) , ty :: Array (t , n)) => n . try_to_target_usize (cx . tcx) . unwrap_or_default () * approx_ty_size (cx , * t) , (Err (_) , ty :: Adt (def , subst)) if def . is_struct () => def . variants () . iter () . map (| v | { v . fields . iter () . map (| field | approx_ty_size (cx , field . ty (cx . tcx , subst))) . sum :: < u64 > () }) . sum () , (Err (_) , ty :: Adt (def , subst)) if def . is_enum () => def . variants () . iter () . map (| v | { v . fields . iter () . map (| field | approx_ty_size (cx , field . ty (cx . tcx , subst))) . sum :: < u64 > () }) . max () . unwrap_or_default () , (Err (_) , ty :: Adt (def , subst)) if def . is_union () => def . variants () . iter () . map (| v | { v . fields . iter () . map (| field | approx_ty_size (cx , field . ty (cx . tcx , subst))) . max () . unwrap_or_default () }) . max () . unwrap_or_default () , (Err (_) , _) => 0 , } }
};
}
