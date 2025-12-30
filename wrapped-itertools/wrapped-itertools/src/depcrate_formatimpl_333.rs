// Generated macro for impl_333 (impl)
macro_rules! Depcrate_formatimpl_333 {
() => {
// Module: crate::format
// Provides: {"impl_333"}
// Dependencies: {}
impl < I , F > fmt :: Display for FormatWith < '_ , I , F > where I : Iterator , F : FnMut (I :: Item , & mut dyn FnMut (& dyn fmt :: Display) -> fmt :: Result) -> fmt :: Result , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let (mut iter , mut format) = match self . inner . take () { Some (t) => t , None => panic ! ("FormatWith: was already formatted once") , } ; if let Some (fst) = iter . next () { format (fst , & mut | disp : & dyn fmt :: Display | disp . fmt (f)) ? ; iter . try_for_each (| elt | { if ! self . sep . is_empty () { f . write_str (self . sep) ? ; } format (elt , & mut | disp : & dyn fmt :: Display | disp . fmt (f)) }) ? ; } Ok (()) } }
};
}
