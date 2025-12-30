// Generated macro for has_bound (function)
macro_rules! Depcrate_boundhas_bound {
() => {
// Module: crate::bound
// Provides: {"has_bound"}
// Dependencies: {}
pub fn has_bound (supertraits : & Supertraits , bound : & InferredBound) -> bool { for supertrait in supertraits { if let TypeParamBound :: Trait (supertrait) = supertrait { if supertrait . path . is_ident (bound) || supertrait . path . segments . len () == 3 && (supertrait . path . segments [0] . ident == "std" || supertrait . path . segments [0] . ident == "core") && supertrait . path . segments [1] . ident == "marker" && supertrait . path . segments [2] . ident == * bound { return true ; } } } false }
};
}
