// Generated macro for impl_335 (impl)
macro_rules! Depcrate_formatimpl_335 {
() => {
// Module: crate::format
// Provides: {"impl_335"}
// Dependencies: {}
impl < I > Format < '_ , I > where I : Iterator , { fn format (& self , f : & mut fmt :: Formatter , cb : fn (& I :: Item , & mut fmt :: Formatter) -> fmt :: Result ,) -> fmt :: Result { let mut iter = match self . inner . take () { Some (t) => t , None => panic ! ("Format: was already formatted once") , } ; if let Some (fst) = iter . next () { cb (& fst , f) ? ; iter . try_for_each (| elt | { if ! self . sep . is_empty () { f . write_str (self . sep) ? ; } cb (& elt , f) }) ? ; } Ok (()) } }
};
}
