macro_rules! deps {
    () => {
        Object!();
        Registry!();
        MetaType!();
        Context!();
        SubscriptionType!();
        MergedObjectTail!();
        Response!();
    };
}

macro_rules! impl_800 {
    () => {
        deps!();
        impl SubscriptionType for MergedObjectTail { fn type_name () -> Cow < 'static , str > { Cow :: Borrowed ("MergedSubscriptionTail") } fn create_type_info (registry : & mut Registry) -> String { registry . create_subscription_type :: < Self , _ > (| _ | MetaType :: Object { name : "MergedSubscriptionTail" . to_string () , description : None , fields : Default :: default () , cache_control : Default :: default () , extends : false , shareable : false , resolvable : true , keys : None , visible : None , inaccessible : false , interface_object : false , tags : Default :: default () , is_subscription : false , rust_typename : Some (std :: any :: type_name :: < Self > ()) , directive_invocations : Default :: default () , requires_scopes : Default :: default () , }) } fn create_field_stream < 'a > (& 'a self , _ctx : & 'a Context < '_ > ,) -> Option < Pin < Box < dyn Stream < Item = Response > + Send + 'a > > > { unreachable ! () } }
    };
}

impl_800!();