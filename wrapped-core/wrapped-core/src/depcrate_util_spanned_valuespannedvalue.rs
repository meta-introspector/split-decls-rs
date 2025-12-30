// Generated macro for SpannedValue (struct)
macro_rules! Depcrate_util_spanned_valueSpannedValue {
() => {
// Module: crate::util::spanned_value
// Provides: {"SpannedValue"}
// Dependencies: {}
# [doc = " A value and an associated position in source code. The main use case for this is"] # [doc = " to preserve position information to emit warnings from proc macros. You can use"] # [doc = " a `SpannedValue<T>` as a field in any struct that implements or derives any of"] # [doc = " `darling`'s core traits."] # [doc = ""] # [doc = " To access the underlying value, use the struct's `Deref` implementation."] # [doc = ""] # [doc = " # Defaulting"] # [doc = " This type is meant to be used in conjunction with attribute-extracted options,"] # [doc = " but the user may not always explicitly set those options in their source code."] # [doc = " In this case, using `Default::default()` will create an instance which points"] # [doc = " to `Span::call_site()`."] # [derive (Debug , Clone , Copy)] pub struct SpannedValue < T > { value : T , span : Span , }
};
}
