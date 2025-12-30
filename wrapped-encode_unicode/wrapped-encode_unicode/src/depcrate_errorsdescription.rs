// Generated macro for description (macro)
macro_rules! Depcrate_errorsdescription {
() => {
// Module: crate::errors
// Provides: {"description"}
// Dependencies: {}
macro_rules ! description { ($ err : ty , $ desc : expr) => { # [cfg (not (feature = "std"))] impl $ err { # [allow (missing_docs)] pub fn description (& self) -> &'static str { ($ desc) (self) } } # [cfg (feature = "std")] impl Error for $ err { fn description (& self) -> &'static str { ($ desc) (self) } } impl Display for $ err { fn fmt (& self , fmtr : & mut Formatter) -> fmt :: Result { #! [allow (deprecated)] write ! (fmtr , "{}" , self . description ()) } } } }
};
}
