// Generated macro for impl_1142 (impl)
macro_rules! Depcrate_types_merged_objectimpl_1142 {
() => {
// Module: crate::types::merged_object
// Provides: {"impl_1142"}
// Dependencies: {}
impl SubscriptionType for MergedObjectTail { fn type_name () -> Cow < 'static , str > { Cow :: Borrowed ("MergedSubscriptionTail") } fn create_type_info (registry : & mut Registry) -> String { registry . create_subscription_type :: < Self , _ > (| _ | MetaType :: Object { name : "MergedSubscriptionTail" . to_string () , description : None , fields : Default :: default () , cache_control : Default :: default () , extends : false , shareable : false , resolvable : true , keys : None , visible : None , inaccessible : false , interface_object : false , tags : Default :: default () , is_subscription : false , rust_typename : Some (std :: any :: type_name :: < Self > ()) , directive_invocations : Default :: default () , requires_scopes : Default :: default () , }) } fn create_field_stream < 'a > (& 'a self , _ctx : & 'a Context < '_ > ,) -> Option < Pin < Box < dyn Stream < Item = Response > + Send + 'a > > > { unreachable ! () } }
};
}
