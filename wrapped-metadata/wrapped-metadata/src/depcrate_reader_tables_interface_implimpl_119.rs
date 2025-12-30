// Generated macro for impl_119 (impl)
macro_rules! Depcrate_reader_tables_interface_implimpl_119 {
() => {
// Module: crate::reader::tables::interface_impl
// Provides: {"impl_119"}
// Dependencies: {}
impl < 'a > InterfaceImpl < 'a > { pub fn class (& self) -> TypeDef < 'a > { self . row (0) } pub fn interface (& self , generics : & [Type]) -> Type { self . decode :: < TypeDefOrRef > (1) . ty (generics) } }
};
}
