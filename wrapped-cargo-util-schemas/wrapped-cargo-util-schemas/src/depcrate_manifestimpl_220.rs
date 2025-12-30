// Generated macro for impl_220 (impl)
macro_rules! Depcrate_manifestimpl_220 {
() => {
// Module: crate::manifest
// Provides: {"impl_220"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for InheritableLints { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { struct InheritableLintsVisitor ; impl < 'de > de :: Visitor < 'de > for InheritableLintsVisitor { type Value = InheritableLints ; fn expecting (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { formatter . write_str ("a lints table") } fn visit_map < M > (self , mut access : M) -> Result < Self :: Value , M :: Error > where M : de :: MapAccess < 'de > , { let mut lints = TomlLints :: new () ; let mut workspace = false ; while let Some (key) = access . next_key () ? { if key == "workspace" { workspace = match access . next_value () ? { Some (WorkspaceValue) => true , None => false , } ; } else { let value = access . next_value () ? ; lints . insert (key , value) ; } } Ok (InheritableLints { workspace , lints }) } } deserializer . deserialize_map (InheritableLintsVisitor) } }
};
}
