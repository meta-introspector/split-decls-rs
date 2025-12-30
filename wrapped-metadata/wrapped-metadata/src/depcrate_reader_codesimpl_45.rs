// Generated macro for impl_45 (impl)
macro_rules! Depcrate_reader_codesimpl_45 {
() => {
// Module: crate::reader::codes
// Provides: {"impl_45"}
// Dependencies: {}
impl TypeDefOrRef < '_ > { pub fn namespace (& self) -> & str { match self { Self :: TypeDef (row) => row . namespace () , Self :: TypeRef (row) => row . namespace () , rest => panic ! ("{rest:?}") , } } pub fn name (& self) -> & str { match self { Self :: TypeDef (row) => row . name () , Self :: TypeRef (row) => row . name () , rest => panic ! ("{rest:?}") , } } pub fn ty (& self , generics : & [Type]) -> Type { if let Self :: TypeSpec (def) = self { return def . ty (generics) ; } Type :: named (self . namespace () , self . name ()) } }
};
}
