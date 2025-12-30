// Generated macro for impl_1115 (impl)
macro_rules! Depcrate_types_jsonimpl_1115 {
() => {
// Module: crate::types::json
// Provides: {"impl_1115"}
// Dependencies: {}
# [cfg_attr (feature = "boxed-trait" , async_trait :: async_trait)] impl < T : Serialize + Send + Sync > OutputType for Json < T > { fn type_name () -> Cow < 'static , str > { Cow :: Borrowed ("JSON") } fn create_type_info (registry : & mut Registry) -> String { registry . create_output_type :: < Json < T > , _ > (MetaTypeId :: Scalar , | _ | MetaType :: Scalar { name : < Self as OutputType > :: type_name () . to_string () , description : Some ("A scalar that can represent any JSON value." . to_string ()) , is_valid : None , visible : None , inaccessible : false , tags : Default :: default () , specified_by_url : None , directive_invocations : Default :: default () , requires_scopes : Default :: default () , }) } async fn resolve (& self , _ctx : & ContextSelectionSet < '_ > , _field : & Positioned < Field > ,) -> ServerResult < Value > { Ok (to_value (& self . 0) . ok () . unwrap_or_default ()) } }
};
}
