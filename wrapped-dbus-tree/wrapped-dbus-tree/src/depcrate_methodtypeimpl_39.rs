// Generated macro for impl_39 (impl)
macro_rules! Depcrate_methodtypeimpl_39 {
() => {
// Module: crate::methodtype
// Provides: {"impl_39"}
// Dependencies: {}
impl < 'a , M : 'a + MethodType < D > , D : 'a + DataType > MethodInfo < 'a , M , D > { # [doc = " MethodInfo to PropInfo conversion"] pub fn to_prop_info (& self , iface : & 'a Interface < M , D > , prop : & 'a Property < M , D >) -> PropInfo < 'a , M , D > { PropInfo { msg : self . msg , method : self . method , iface : iface , prop : prop , path : self . path , tree : self . tree } } }
};
}
