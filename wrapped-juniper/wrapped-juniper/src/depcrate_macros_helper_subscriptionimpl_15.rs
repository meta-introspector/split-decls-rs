// Generated macro for impl_15 (impl)
macro_rules! Depcrate_macros_helper_subscriptionimpl_15 {
() => {
// Module: crate::macros::helper::subscription
// Provides: {"impl_15"}
// Dependencies: {}
impl < T , E , S > IntoFieldResult < T , S > for Result < T , E > where T : IntoFieldResult < T , S > , E : IntoFieldError < S > , { type Item = T :: Item ; fn into_result (self) -> Result < T , FieldError < S > > { self . map_err (E :: into_field_error) } }
};
}
