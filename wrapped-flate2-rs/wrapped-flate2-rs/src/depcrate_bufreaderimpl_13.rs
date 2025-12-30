// Generated macro for impl_13 (impl)
macro_rules! Depcrate_bufreaderimpl_13 {
() => {
// Module: crate::bufreader
// Provides: {"impl_13"}
// Dependencies: {}
impl < R > :: std :: fmt :: Debug for BufReader < R > where R : :: std :: fmt :: Debug , { fn fmt (& self , fmt : & mut :: std :: fmt :: Formatter) -> Result < () , :: std :: fmt :: Error > { fmt . debug_struct ("BufReader") . field ("reader" , & self . inner) . field ("buffer" , & format_args ! ("{}/{}" , self . cap - self . pos , self . buf . len ()) ,) . finish () } }
};
}
