// Generated macro for impl_1139 (impl)
macro_rules! Depcrate_types_merged_objectimpl_1139 {
() => {
// Module: crate::types::merged_object
// Provides: {"impl_1139"}
// Dependencies: {}
# [cfg_attr (feature = "boxed-trait" , async_trait :: async_trait)] impl < A , B > OutputType for MergedObject < A , B > where A : OutputType , B : OutputType , { fn type_name () -> Cow < 'static , str > { Cow :: Owned (format ! ("{}_{}" , A :: type_name () , B :: type_name ())) } fn create_type_info (registry : & mut Registry) -> String { registry . create_output_type :: < Self , _ > (MetaTypeId :: Object , | registry | { let mut fields = IndexMap :: new () ; let mut cc = CacheControl :: default () ; if let MetaType :: Object { fields : b_fields , cache_control : b_cc , .. } = registry . create_fake_output_type :: < B > () { fields . extend (b_fields) ; cc = cc . merge (& b_cc) ; } if let MetaType :: Object { fields : a_fields , cache_control : a_cc , .. } = registry . create_fake_output_type :: < A > () { fields . extend (a_fields) ; cc = cc . merge (& a_cc) ; } MetaType :: Object { name : Self :: type_name () . to_string () , description : None , fields , cache_control : cc , extends : false , shareable : false , resolvable : true , keys : None , visible : None , inaccessible : false , interface_object : false , tags : Default :: default () , is_subscription : false , rust_typename : Some (std :: any :: type_name :: < Self > ()) , directive_invocations : Default :: default () , requires_scopes : Default :: default () , } }) } async fn resolve (& self , _ctx : & ContextSelectionSet < '_ > , _field : & Positioned < Field > ,) -> ServerResult < Value > { unreachable ! () } }
};
}
