// Generated macro for impl_49 (impl)
macro_rules! Depcrate_frameimpl_49 {
() => {
// Module: crate::frame
// Provides: {"impl_49"}
// Dependencies: {}
impl fmt :: Display for DisplayFrame < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let level = if let Some (level) = self . frame . level { let level = if self . colored { match level { Level :: Trace => "TRACE" . dimmed () . to_string () , Level :: Debug => "DEBUG" . normal () . to_string () , Level :: Info => "INFO" . green () . to_string () , Level :: Warn => "WARN" . yellow () . to_string () , Level :: Error => "ERROR" . red () . to_string () , } } else { match level { Level :: Trace => "TRACE" . to_string () , Level :: Debug => "DEBUG" . to_string () , Level :: Info => "INFO" . to_string () , Level :: Warn => "WARN" . to_string () , Level :: Error => "ERROR" . to_string () , } } ; format ! ("{level} ") } else { "" . to_string () } ; let timestamp = self . frame . timestamp_format . map (| fmt | { format ! ("{} " , self . frame . format_args (fmt , & self . frame . timestamp_args , None ,) ,) }) . unwrap_or_default () ; let args = self . frame . format_args (self . frame . format , & self . frame . args , None) ; write ! (f , "{timestamp}{level}{args}") } }
};
}
