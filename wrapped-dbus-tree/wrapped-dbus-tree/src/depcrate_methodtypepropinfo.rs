// Generated macro for PropInfo (struct)
macro_rules! Depcrate_methodtypePropInfo {
() => {
// Module: crate::methodtype
// Provides: {"PropInfo"}
// Dependencies: {}
# [derive (Debug , Copy , Clone)] # [doc = " Contains information about the incoming property get/set request."] pub struct PropInfo < 'a , M : 'a + MethodType < D > , D : 'a + DataType > { # [doc = " Message"] pub msg : & 'a Message , # [doc = " Get, Set or GetAll"] pub method : & 'a Method < M , D > , # [doc = " The property to be set/get"] pub prop : & 'a Property < M , D > , # [doc = " The interface the property belongs to"] pub iface : & 'a Interface < M , D > , # [doc = " Object path"] pub path : & 'a ObjectPath < M , D > , # [doc = " Tree"] pub tree : & 'a Tree < M , D > , }
};
}
