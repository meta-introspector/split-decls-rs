// Generated macro for impl_9 (impl)
macro_rules! Depcrate_capi_datetimeimpl_9 {
() => {
// Module: crate::capi_datetime
// Provides: {"impl_9"}
// Dependencies: {}
impl ConstructorType { pub const VALUES : & 'static [Self] = & [Self :: CompiledData , Self :: WithProvider] ; pub fn is_with_provider (self) -> bool { matches ! (self , Self :: WithProvider) } pub fn prefix (self) -> & 'static str { match self { Self :: CompiledData => "include_" , Self :: WithProvider => "load_" , } } pub fn suffix_ffi (self) -> & 'static str { match self { Self :: CompiledData => "" , Self :: WithProvider => "_with_provider" , } } pub fn suffix_rust (self) -> & 'static str { match self { Self :: CompiledData => "" , Self :: WithProvider => "_with_buffer_provider" , } } pub fn cargo_feature (self) -> & 'static str { match self { Self :: CompiledData => "compiled_data" , Self :: WithProvider => "buffer_provider" , } } }
};
}
