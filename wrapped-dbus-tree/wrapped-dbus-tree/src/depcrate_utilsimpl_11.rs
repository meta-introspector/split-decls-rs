// Generated macro for impl_11 (impl)
macro_rules! Depcrate_utilsimpl_11 {
() => {
// Module: crate::utils
// Provides: {"impl_11"}
// Dependencies: {}
impl Argument { # [doc = " Create a new Argument."] pub fn new (name : Option < String > , sig : Signature < 'static >) -> Argument { Argument (name , sig) } # [doc = " Descriptive name (if any)."] pub fn name (& self) -> Option < & str > { self . 0 . as_ref () . map (| s | & * * s) } # [doc = " Type signature of argument."] pub fn signature (& self) -> & Signature < 'static > { & self . 1 } fn introspect (& self , indent : & str , dir : & str) -> String { let n = self . 0 . as_ref () . map (| n | format ! ("name=\"{}\" " , n)) . unwrap_or_default () ; format ! ("{}<arg {}type=\"{}\"{}/>\n" , indent , n , self . 1 , dir) } }
};
}
