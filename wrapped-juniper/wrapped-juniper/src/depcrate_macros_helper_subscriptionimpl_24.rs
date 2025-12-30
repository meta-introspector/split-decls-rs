// Generated macro for impl_24 (impl)
macro_rules! Depcrate_macros_helper_subscriptionimpl_24 {
() => {
// Module: crate::macros::helper::subscription
// Provides: {"impl_24"}
// Dependencies: {}
impl < T , I , E , S > ExtractTypeFromStream < ResultStreamItem , S > for Result < T , E > where T : futures :: Stream < Item = I > , I : GraphQLValue < S > , S : ScalarValue , { type Item = I ; }
};
}
