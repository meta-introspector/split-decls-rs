// Generated macro for impl_294 (impl)
macro_rules! Depcrate_error_kindimpl_294 {
() => {
// Module: crate::error::kind
// Provides: {"impl_294"}
// Dependencies: {}
impl fmt :: Display for ErrorUnknownValue { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "Unknown {}: `{}`" , self . noun , self . value) ? ; if let Some ((_ , ref did_you_mean)) = self . did_you_mean { write ! (f , ". Did you mean `{}`?" , did_you_mean) ? ; } else if ! self . alts . is_empty () && self . alts . len () < 10 { write ! (f , ". Available values: ") ? ; write_delimited (f , self . alts . iter () . map (Quoted :: backticks) , ", ") ? ; } Ok (()) } }
};
}
