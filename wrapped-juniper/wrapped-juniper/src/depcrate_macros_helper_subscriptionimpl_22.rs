// Generated macro for impl_22 (impl)
macro_rules! Depcrate_macros_helper_subscriptionimpl_22 {
() => {
// Module: crate::macros::helper::subscription
// Provides: {"impl_22"}
// Dependencies: {}
impl < T , I , S > ExtractTypeFromStream < StreamItem , S > for T where T : futures :: Stream < Item = I > , I : GraphQLValue < S > , S : ScalarValue , { type Item = I ; }
};
}
