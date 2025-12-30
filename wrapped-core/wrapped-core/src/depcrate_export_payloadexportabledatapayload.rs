// Generated macro for ExportableDataPayload (trait)
macro_rules! Depcrate_export_payloadExportableDataPayload {
() => {
// Module: crate::export::payload
// Provides: {"ExportableDataPayload"}
// Dependencies: {}
trait ExportableDataPayload { fn bake_yoke (& self , ctx : & CrateEnv) -> TokenStream ; fn bake_size (& self) -> usize ; fn serialize_yoke (& self , serializer : & mut dyn erased_serde :: Serializer ,) -> Result < () , DataError > ; fn maybe_bake_varule_encoded (& self , rest : & [& DataPayload < ExportMarker >] , ctx : & CrateEnv ,) -> Option < TokenStream > ; fn as_any (& self) -> & dyn Any ; fn eq_dyn (& self , other : & dyn ExportableDataPayload) -> bool ; }
};
}
