// Generated macro for impl_175 (impl)
macro_rules! Depcrate_schemaimpl_175 {
() => {
// Module: crate::schema
// Provides: {"impl_175"}
// Dependencies: {}
impl < Query , Mutation , Subscription > Default for Schema < Query , Mutation , Subscription > where Query : Default + ObjectType + 'static , Mutation : Default + ObjectType + 'static , Subscription : Default + SubscriptionType + 'static , { fn default () -> Self { Schema :: new (Query :: default () , Mutation :: default () , Subscription :: default () ,) } }
};
}
