// Generated macro for impl_3061 (impl)
macro_rules! Depcrate_item_name_repetitionsimpl_3061 {
() => {
// Module: crate::item_name_repetitions
// Provides: {"impl_3061"}
// Dependencies: {}
impl ItemNameRepetitions { pub fn new (conf : & 'static Conf) -> Self { Self { modules : Vec :: new () , enum_threshold : conf . enum_variant_name_threshold , struct_threshold : conf . struct_field_name_threshold , avoid_breaking_exported_api : conf . avoid_breaking_exported_api , allow_exact_repetitions : conf . allow_exact_repetitions , allow_private_module_inception : conf . allow_private_module_inception , allowed_prefixes : conf . allowed_prefixes . iter () . map (| s | to_camel_case (s)) . collect () , } } fn is_allowed_prefix (& self , prefix : & str) -> bool { self . allowed_prefixes . contains (prefix) } }
};
}
