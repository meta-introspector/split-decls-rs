// Generated macro for impl_119 (impl)
macro_rules! Depcrate_defimpl_119 {
() => {
// Module: crate::def
// Provides: {"impl_119"}
// Dependencies: {}
impl CtorKind { pub fn from_ast (vdata : & ast :: VariantData) -> Option < (CtorKind , NodeId) > { match * vdata { ast :: VariantData :: Tuple (_ , node_id) => Some ((CtorKind :: Fn , node_id)) , ast :: VariantData :: Unit (node_id) => Some ((CtorKind :: Const , node_id)) , ast :: VariantData :: Struct { .. } => None , } } }
};
}
