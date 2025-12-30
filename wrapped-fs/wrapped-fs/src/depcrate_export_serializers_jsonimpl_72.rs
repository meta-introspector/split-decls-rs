// Generated macro for impl_72 (impl)
macro_rules! Depcrate_export_serializers_jsonimpl_72 {
() => {
// Module: crate::export::serializers::json
// Provides: {"impl_72"}
// Dependencies: {}
impl Serializer { # [doc = " Creates a new serializer for [`serde_json`]."] pub fn new (options : Options) -> Self { Self { style : options . style , } } # [doc = " Convenience function to create a JSON serializer with the"] # [doc = " [`StyleOption::Pretty`] format."] pub fn pretty () -> Self { Self :: new (Options { style : StyleOption :: Pretty , .. Default :: default () }) } }
};
}
