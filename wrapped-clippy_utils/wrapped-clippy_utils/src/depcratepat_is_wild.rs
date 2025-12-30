// Generated macro for pat_is_wild (function)
macro_rules! Depcratepat_is_wild {
() => {
// Module: crate
// Provides: {"pat_is_wild"}
// Dependencies: {}
# [doc = " Returns `true` if the pattern is a `PatWild`, or is an ident prefixed with `_`"] # [doc = " that is not locally used."] pub fn pat_is_wild < 'tcx > (cx : & LateContext < 'tcx > , pat : & 'tcx PatKind < '_ > , body : impl Visitable < 'tcx >) -> bool { match * pat { PatKind :: Wild => true , PatKind :: Binding (_ , id , ident , None) if ident . as_str () . starts_with ('_') => { ! visitors :: is_local_used (cx , body , id) } , _ => false , } }
};
}
