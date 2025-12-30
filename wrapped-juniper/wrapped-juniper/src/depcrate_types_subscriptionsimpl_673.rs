// Generated macro for impl_673 (impl)
macro_rules! Depcrate_types_subscriptionsimpl_673 {
() => {
// Module: crate::types::subscriptions
// Provides: {"impl_673"}
// Dependencies: {}
impl < S , T > GraphQLSubscriptionType < S > for T where T : GraphQLSubscriptionValue < S > + GraphQLType < S > + ? Sized , T :: Context : Sync , T :: TypeInfo : Sync , S : ScalarValue + Send + Sync , { }
};
}
