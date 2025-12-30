// Generated macro for MethodInfo (struct)
macro_rules! Depcrate_methodtypeMethodInfo {
() => {
// Module: crate::methodtype
// Provides: {"MethodInfo"}
// Dependencies: {}
# [derive (Debug , Copy , Clone)] # [doc = " Contains information about the incoming method call."] pub struct MethodInfo < 'a , M : 'a + MethodType < D > , D : 'a + DataType > { # [doc = " Message"] pub msg : & 'a Message , # [doc = " The method to be called"] pub method : & 'a Method < M , D > , # [doc = " Interface"] pub iface : & 'a Interface < M , D > , # [doc = " Object path"] pub path : & 'a ObjectPath < M , D > , # [doc = " Tree"] pub tree : & 'a Tree < M , D > , }
};
}
