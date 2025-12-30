// Generated macro for impl_52 (impl)
macro_rules! Depcrate_match_group_validateimpl_52 {
() => {
// Module: crate::match_group::validate
// Provides: {"impl_52"}
// Dependencies: {}
impl std :: fmt :: Display for Error { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "Found {} {} the refspec mapping to be used: \n\t{}" , self . issues . len () , if self . issues . len () == 1 { "issue that prevents" } else { "issues that prevent" } , self . issues . iter () . map (ToString :: to_string) . collect ::< Vec < _ >> () . join ("\n\t")) } }
};
}
