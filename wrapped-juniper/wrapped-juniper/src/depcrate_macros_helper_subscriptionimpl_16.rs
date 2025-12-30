// Generated macro for impl_16 (impl)
macro_rules! Depcrate_macros_helper_subscriptionimpl_16 {
() => {
// Module: crate::macros::helper::subscription
// Provides: {"impl_16"}
// Dependencies: {}
impl < T , S > IntoFieldResult < T , S > for T where T : Stream , { type Item = T :: Item ; fn into_result (self) -> Result < T , FieldError < S > > { Ok (self) } }
};
}
