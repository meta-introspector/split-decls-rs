// Generated macro for IntoFieldResult (trait)
macro_rules! Depcrate_macros_helper_subscriptionIntoFieldResult {
() => {
// Module: crate::macros::helper::subscription
// Provides: {"IntoFieldResult"}
// Dependencies: {}
# [doc = " Trait for wrapping [`Stream`] into [`Ok`] if it's not [`Result`]."] # [doc = ""] # [doc = " Used in subscription macros when user can provide type alias for [`Stream`] or"] # [doc = " `Result<Stream, _>` and then a function on [`Stream`] should be called."] pub trait IntoFieldResult < T , S > { # [doc = " Type of items yielded by this [`Stream`]."] type Item ; # [doc = " Turns current [`Stream`] type into a generic [`Result`]."] fn into_result (self) -> Result < T , FieldError < S > > ; }
};
}
