// Generated macro for impl_288 (impl)
macro_rules! Depcrate_configimpl_288 {
() => {
// Module: crate::config
// Provides: {"impl_288"}
// Dependencies: {}
impl LensConfig { pub fn any (& self) -> bool { self . run || self . debug || self . update_test || self . implementations || self . method_refs || self . refs_adt || self . refs_trait || self . enum_variant_refs } pub fn none (& self) -> bool { ! self . any () } pub fn runnable (& self) -> bool { self . run || self . debug || self . update_test } pub fn references (& self) -> bool { self . method_refs || self . refs_adt || self . refs_trait || self . enum_variant_refs } pub fn into_annotation_config < 'a > (self , binary_target : bool , minicore : MiniCore < 'a > ,) -> AnnotationConfig < 'a > { AnnotationConfig { binary_target , annotate_runnables : self . runnable () , annotate_impls : self . implementations , annotate_references : self . refs_adt , annotate_method_references : self . method_refs , annotate_enum_variant_references : self . enum_variant_refs , location : self . location . into () , minicore , filter_adjacent_derive_implementations : self . filter_adjacent_derive_implementations , } } }
};
}
