// Generated macro for impl_280 (impl)
macro_rules! Depcrate_errorimpl_280 {
() => {
// Module: crate::error
// Provides: {"impl_280"}
// Dependencies: {}
impl core :: fmt :: Display for Error { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { # [cfg (feature = "alloc")] { let mut err = self ; loop { let Some (ref inner) = err . inner else { write ! (f , "unknown jiff error") ? ; break ; } ; write ! (f , "{}" , inner . kind) ? ; err = match inner . cause . as_ref () { None => break , Some (err) => err , } ; write ! (f , ": ") ? ; } Ok (()) } # [cfg (not (feature = "alloc"))] { match self . inner { None => write ! (f , "unknown jiff error") , Some (ref inner) => write ! (f , "{}" , inner . kind) , } } } }
};
}
