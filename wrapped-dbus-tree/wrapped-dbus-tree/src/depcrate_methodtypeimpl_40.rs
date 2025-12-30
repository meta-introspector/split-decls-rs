// Generated macro for impl_40 (impl)
macro_rules! Depcrate_methodtypeimpl_40 {
() => {
// Module: crate::methodtype
// Provides: {"impl_40"}
// Dependencies: {}
impl < 'a , M : 'a + MethodType < D > , D : 'a + DataType > stdintf :: OrgFreedesktopDBusIntrospectable for MethodInfo < 'a , M , D > { type Err = MethodErr ; fn introspect (& self) -> Result < String , Self :: Err > { Ok (self . path . introspect (self . tree)) } }
};
}
