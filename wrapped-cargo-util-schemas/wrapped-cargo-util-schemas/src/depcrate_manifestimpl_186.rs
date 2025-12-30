// Generated macro for impl_186 (impl)
macro_rules! Depcrate_manifestimpl_186 {
() => {
// Module: crate::manifest
// Provides: {"impl_186"}
// Dependencies: {}
impl ser :: Serialize for TomlDebugInfo { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : ser :: Serializer , { match self { Self :: None => 0 . serialize (serializer) , Self :: LineDirectivesOnly => "line-directives-only" . serialize (serializer) , Self :: LineTablesOnly => "line-tables-only" . serialize (serializer) , Self :: Limited => 1 . serialize (serializer) , Self :: Full => 2 . serialize (serializer) , } } }
};
}
