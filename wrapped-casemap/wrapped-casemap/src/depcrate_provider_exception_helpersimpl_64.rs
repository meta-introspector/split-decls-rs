// Generated macro for impl_64 (impl)
macro_rules! Depcrate_provider_exception_helpersimpl_64 {
() => {
// Module: crate::provider::exception_helpers
// Provides: {"impl_64"}
// Dependencies: {}
impl ExceptionBits { # [doc = " Extract from the upper half of an ICU4C-format u16"] pub (crate) fn from_integer (int : u8) -> Self { let ule = ExceptionBitsULE (int) ; let double_width_slots = ule . double_width_slots () ; let no_simple_case_folding = ule . no_simple_case_folding () ; let negative_delta = ule . negative_delta () ; let is_sensitive = ule . is_sensitive () ; let has_conditional_special = ule . has_conditional_special () ; let has_conditional_fold = ule . has_conditional_fold () ; let dot_type = ule . dot_type () ; Self { double_width_slots , no_simple_case_folding , negative_delta , is_sensitive , dot_type , has_conditional_special , has_conditional_fold , } } # [doc = " Convert to an ICU4C-format upper half of u16"] pub (crate) fn to_integer (self) -> u8 { let mut int = 0 ; let dot_data = (self . dot_type as u8) << ExceptionBitsULE :: DOT_SHIFT ; int |= dot_data ; if self . double_width_slots { int |= ExceptionBitsULE :: DOUBLE_SLOTS_FLAG } if self . no_simple_case_folding { int |= ExceptionBitsULE :: NO_SIMPLE_CASE_FOLDING_FLAG } if self . negative_delta { int |= ExceptionBitsULE :: NEGATIVE_DELTA_FLAG } if self . is_sensitive { int |= ExceptionBitsULE :: SENSITIVE_FLAG } if self . has_conditional_special { int |= ExceptionBitsULE :: CONDITIONAL_SPECIAL_FLAG } if self . has_conditional_fold { int |= ExceptionBitsULE :: CONDITIONAL_FOLD_FLAG } int } }
};
}
