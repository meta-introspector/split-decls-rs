// Generated macro for impl_70 (impl)
macro_rules! Depcrate_utils_authorimpl_70 {
() => {
// Module: crate::utils::author
// Provides: {"impl_70"}
// Dependencies: {}
impl < T : Display > Display for OptionPat < T > { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { match & self . opt { None => f . write_str ("None") , Some (node) => write ! (f , "Some({node})") , } } }
};
}
