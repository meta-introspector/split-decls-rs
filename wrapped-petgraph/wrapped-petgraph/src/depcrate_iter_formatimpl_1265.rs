// Generated macro for impl_1265 (impl)
macro_rules! Depcrate_iter_formatimpl_1265 {
() => {
// Module: crate::iter_format
// Provides: {"impl_1265"}
// Dependencies: {}
impl < I > Format < '_ , I > where I : Iterator , { fn format < F > (& self , f : & mut fmt :: Formatter , mut cb : F) -> fmt :: Result where F : FnMut (& I :: Item , & mut fmt :: Formatter) -> fmt :: Result , { let mut iter = match self . inner . borrow_mut () . take () { Some (t) => t , None => panic ! ("Format: was already formatted once") , } ; if let Some (fst) = iter . next () { cb (& fst , f) ? ; for elt in iter { if ! self . sep . is_empty () { f . write_str (self . sep) ? ; } cb (& elt , f) ? ; } } Ok (()) } }
};
}
