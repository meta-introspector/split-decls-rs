// Generated macro for impl_94 (impl)
macro_rules! Depcrate_blob_platform_resourceimpl_94 {
() => {
// Module: crate::blob::platform::resource
// Provides: {"impl_94"}
// Dependencies: {}
impl < 'a > ResourceRef < 'a > { pub (super) fn new (cache : & 'a Resource) -> Self { ResourceRef { data : cache . data . map_or (Data :: Missing , | data | match data { pipeline :: Data :: Buffer => Data :: Buffer (& cache . buffer) , pipeline :: Data :: TooLarge { size } => Data :: TooLarge { size } , }) , rela_path : cache . rela_path . as_ref () , id : & cache . id , } } }
};
}
