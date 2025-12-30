// Generated macro for impl_752 (impl)
macro_rules! Depcrate_debuginfo_metadataimpl_752 {
() => {
// Module: crate::debuginfo::metadata
// Provides: {"impl_752"}
// Dependencies: {}
impl MsvcBasicName for ty :: FloatTy { fn msvc_basic_name (self) -> & 'static str { match self { ty :: FloatTy :: F16 => { bug ! ("`f16` should have been handled in `build_basic_type_di_node`") } ty :: FloatTy :: F32 => "float" , ty :: FloatTy :: F64 => "double" , ty :: FloatTy :: F128 => "fp128" , } } }
};
}
