// Generated macro for impl_25 (impl)
macro_rules! Depcrate_macros_helper_subscriptionimpl_25 {
() => {
// Module: crate::macros::helper::subscription
// Provides: {"impl_25"}
// Dependencies: {}
impl < T , E , I , ER , S > ExtractTypeFromStream < ResultStreamResult , S > for Result < T , E > where T : futures :: Stream < Item = Result < I , ER > > , I : GraphQLValue < S > , S : ScalarValue , { type Item = I ; }
};
}
