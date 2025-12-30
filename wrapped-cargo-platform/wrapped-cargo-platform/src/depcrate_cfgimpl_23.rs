// Generated macro for impl_23 (impl)
macro_rules! Depcrate_cfgimpl_23 {
() => {
// Module: crate::cfg
// Provides: {"impl_23"}
// Dependencies: {}
impl fmt :: Display for Cfg { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Cfg :: Name (ref s) => s . fmt (f) , Cfg :: KeyPair (ref k , ref v) => write ! (f , "{} = \"{}\"" , k , v) , } } }
};
}
