// Generated macro for impl_43 (impl)
macro_rules! Depcrate_methodtypeimpl_43 {
() => {
// Module: crate::methodtype
// Provides: {"impl_43"}
// Dependencies: {}
impl < 'a , M : 'a + MethodType < D > , D : 'a + DataType > PropInfo < 'a , M , D > { # [doc = " PropInfo to MethodInfo conversion."] pub fn to_method_info (& self) -> MethodInfo < 'a , M , D > { MethodInfo { msg : self . msg , method : self . method , iface : self . iface , path : self . path , tree : self . tree } } }
};
}
