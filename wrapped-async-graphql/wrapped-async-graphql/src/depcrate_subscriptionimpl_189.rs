// Generated macro for impl_189 (impl)
macro_rules! Depcrate_subscriptionimpl_189 {
() => {
// Module: crate::subscription
// Provides: {"impl_189"}
// Dependencies: {}
impl < T : SubscriptionType > SubscriptionType for & T { fn type_name () -> Cow < 'static , str > { T :: type_name () } fn create_type_info (registry : & mut Registry) -> String { T :: create_type_info (registry) } fn create_field_stream < 'a > (& 'a self , ctx : & 'a Context < '_ > ,) -> Option < Pin < Box < dyn Stream < Item = Response > + Send + 'a > > > { T :: create_field_stream (* self , ctx) } }
};
}
