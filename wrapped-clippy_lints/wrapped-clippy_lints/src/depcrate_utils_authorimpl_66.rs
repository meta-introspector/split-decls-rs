// Generated macro for impl_66 (impl)
macro_rules! Depcrate_utils_authorimpl_66 {
() => {
// Module: crate::utils::author
// Provides: {"impl_66"}
// Dependencies: {}
impl < T : Display > Display for OptionPat < T > { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { match & self . opt { None => f . write_str ("None") , Some (node) => write ! (f , "Some({node})") , } } }
};
}
