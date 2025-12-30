// Generated macro for impl_1164 (impl)
macro_rules! Depcrate_scaffold_names_storageimpl_1164 {
() => {
// Module: crate::scaffold::names_storage
// Provides: {"impl_1164"}
// Dependencies: {}
impl < Payload > OptionalNames < () , Payload > where Payload : Copy , { pub (crate) fn get_option (& self) -> Option < Payload > { match self { Self :: SingleLength { variables : () , payload , } => Some (* payload) , _ => None , } } }
};
}
