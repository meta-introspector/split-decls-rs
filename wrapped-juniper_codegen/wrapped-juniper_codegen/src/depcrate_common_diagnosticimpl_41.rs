// Generated macro for impl_41 (impl)
macro_rules! Depcrate_common_diagnosticimpl_41 {
() => {
// Module: crate::common::diagnostic
// Provides: {"impl_41"}
// Dependencies: {}
impl Scope { pub (crate) fn spec_section (& self) -> & str { match self { Self :: EnumDerive => "#sec-Enums" , Self :: InputObjectDerive => "#sec-Input-Objects" , Self :: InterfaceAttr | Self :: InterfaceDerive => "#sec-Interfaces" , Self :: ObjectAttr | Self :: ObjectDerive => "#sec-Objects" , Self :: ScalarAttr | Self :: ScalarDerive => "#sec-Scalars" , Self :: ScalarValueDerive => "#sec-Scalars.Built-in-Scalars" , Self :: UnionAttr | Self :: UnionDerive => "#sec-Unions" , } } }
};
}
