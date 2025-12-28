macro_rules! deps {
    () => {
        Response!();
        Context!();
        EmptySubscription!();
        Registry!();
    };
}

macro_rules! SubscriptionType {
    () => {
        deps!();
        # [doc = " A GraphQL subscription object"] pub trait SubscriptionType : Send + Sync { # [doc = " Type the name."] fn type_name () -> Cow < 'static , str > ; # [doc = " Qualified typename."] fn qualified_type_name () -> String { format ! ("{}!" , Self :: type_name ()) } # [doc = " Create type information in the registry and return qualified typename."] fn create_type_info (registry : & mut registry :: Registry) -> String ; # [doc = " This function returns true of type `EmptySubscription` only."] # [doc (hidden)] fn is_empty () -> bool { false } # [doc (hidden)] fn create_field_stream < 'a > (& 'a self , ctx : & 'a Context < '_ > ,) -> Option < Pin < Box < dyn Stream < Item = Response > + Send + 'a > > > ; }
    };
}

SubscriptionType!()