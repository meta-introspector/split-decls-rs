// Generated macro for impl_1117 (impl)
macro_rules! Depcrate_types_jsonimpl_1117 {
() => {
// Module: crate::types::json
// Provides: {"impl_1117"}
// Dependencies: {}
# [cfg_attr (feature = "boxed-trait" , async_trait :: async_trait)] impl OutputType for serde_json :: Value { fn type_name () -> Cow < 'static , str > { Cow :: Borrowed ("JSON") } fn create_type_info (registry : & mut Registry) -> String { registry . create_output_type :: < serde_json :: Value , _ > (MetaTypeId :: Scalar , | _ | { MetaType :: Scalar { name : < Self as OutputType > :: type_name () . to_string () , description : Some ("A scalar that can represent any JSON value." . to_string ()) , is_valid : None , visible : None , inaccessible : false , tags : Default :: default () , specified_by_url : None , directive_invocations : Default :: default () , requires_scopes : Default :: default () , } }) } async fn resolve (& self , _ctx : & ContextSelectionSet < '_ > , _field : & Positioned < Field > ,) -> ServerResult < Value > { Ok (to_value (self) . ok () . unwrap_or_default ()) } }
};
}
