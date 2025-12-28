macro_rules! deps {
    () => {
        MetaType!();
        MergedObject!();
        CacheControl!();
        Response!();
        Object!();
        SubscriptionType!();
        Context!();
        Registry!();
    };
}

macro_rules! impl_798 {
    () => {
        deps!();
        impl < A , B > SubscriptionType for MergedObject < A , B > where A : SubscriptionType , B : SubscriptionType , { fn type_name () -> Cow < 'static , str > { Cow :: Owned (format ! ("{}_{}" , A :: type_name () , B :: type_name ())) } fn create_type_info (registry : & mut Registry) -> String { registry . create_subscription_type :: < Self , _ > (| registry | { let mut fields = IndexMap :: new () ; let mut cc = CacheControl :: default () ; if let MetaType :: Object { fields : b_fields , cache_control : b_cc , .. } = registry . create_fake_subscription_type :: < B > () { fields . extend (b_fields) ; cc = cc . merge (& b_cc) ; } if let MetaType :: Object { fields : a_fields , cache_control : a_cc , .. } = registry . create_fake_subscription_type :: < A > () { fields . extend (a_fields) ; cc = cc . merge (& a_cc) ; } MetaType :: Object { name : Self :: type_name () . to_string () , description : None , fields , cache_control : cc , extends : false , shareable : false , resolvable : true , keys : None , visible : None , inaccessible : false , interface_object : false , tags : Default :: default () , is_subscription : false , rust_typename : Some (std :: any :: type_name :: < Self > ()) , directive_invocations : Default :: default () , requires_scopes : Default :: default () , } }) } fn create_field_stream < 'a > (& 'a self , _ctx : & 'a Context < '_ > ,) -> Option < Pin < Box < dyn Stream < Item = Response > + Send + 'a > > > { unreachable ! () } }
    };
}

impl_798!();