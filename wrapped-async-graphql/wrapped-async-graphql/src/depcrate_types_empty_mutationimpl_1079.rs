// Generated macro for impl_1079 (impl)
macro_rules! Depcrate_types_empty_mutationimpl_1079 {
() => {
// Module: crate::types::empty_mutation
// Provides: {"impl_1079"}
// Dependencies: {}
# [cfg_attr (feature = "boxed-trait" , async_trait :: async_trait)] impl OutputType for EmptyMutation { fn type_name () -> Cow < 'static , str > { Cow :: Borrowed ("EmptyMutation") } fn create_type_info (registry : & mut registry :: Registry) -> String { registry . create_output_type :: < Self , _ > (MetaTypeId :: Object , | _ | registry :: MetaType :: Object { name : "EmptyMutation" . to_string () , description : None , fields : Default :: default () , cache_control : Default :: default () , extends : false , shareable : false , resolvable : true , keys : None , visible : None , inaccessible : false , interface_object : false , tags : Default :: default () , is_subscription : false , rust_typename : Some (std :: any :: type_name :: < Self > ()) , directive_invocations : Default :: default () , requires_scopes : Default :: default () , }) } async fn resolve (& self , _ctx : & ContextSelectionSet < '_ > , _field : & Positioned < Field > ,) -> ServerResult < Value > { Err (ServerError :: new ("Schema is not configured for mutations." , None ,)) } }
};
}
