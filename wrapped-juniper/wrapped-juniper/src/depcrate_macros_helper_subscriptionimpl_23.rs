// Generated macro for impl_23 (impl)
macro_rules! Depcrate_macros_helper_subscriptionimpl_23 {
() => {
// Module: crate::macros::helper::subscription
// Provides: {"impl_23"}
// Dependencies: {}
impl < Ty , T , E , S > ExtractTypeFromStream < StreamResult , S > for Ty where Ty : futures :: Stream < Item = Result < T , E > > , T : GraphQLValue < S > , S : ScalarValue , { type Item = T ; }
};
}
