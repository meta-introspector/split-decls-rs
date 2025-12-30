// Generated macro for impl_171 (impl)
macro_rules! Depcrate_normalizeimpl_171 {
() => {
// Module: crate::normalize
// Provides: {"impl_171"}
// Dependencies: {}
impl < S : Spec > fmt :: Display for NormalizedInner < '_ , S > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . input . op . mode . case_pct_normalization () { normalize_scheme (f , self . input . scheme) ? ; } else { f . write_str (self . input . scheme) ? ; } f . write_str (":") ? ; if let Some (authority) = self . input . authority { f . write_str ("//") ? ; if self . input . op . mode . case_pct_normalization () { normalize_authority :: < S > (f , authority) ? ; } else { f . write_str (authority) ? ; } } match self . input . path { Path :: Done (s) => { if self . input . op . mode . case_pct_normalization () { PathToNormalize :: from_single_path (s) . fmt_write_normalize :: < S , _ > (f , self . input . op , self . input . authority . is_some () ,) ? } else { f . write_str (s) ? } } Path :: NeedsProcessing (path) => { path . fmt_write_normalize :: < S , _ > (f , self . input . op , self . input . authority . is_some ()) ? } } if let Some (query) = self . input . query { f . write_char ('?') ? ; if self . input . op . mode . case_pct_normalization () { normalize_query :: < S > (f , query) ? ; } else { f . write_str (query) ? ; } } if let Some (fragment) = self . input . fragment { f . write_char ('#') ? ; if self . input . op . mode . case_pct_normalization () { normalize_fragment :: < S > (f , fragment) ? ; } else { f . write_str (fragment) ? ; } } Ok (()) } }
};
}
