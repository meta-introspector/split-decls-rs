// Generated macro for impl_69 (impl)
macro_rules! Depcrate_provider_exception_helpersimpl_69 {
() => {
// Module: crate::provider::exception_helpers
// Provides: {"impl_69"}
// Dependencies: {}
impl ExceptionBitsULE { # [doc = " Whether or not the slots are double-width."] # [doc = ""] # [doc = " Unused in ICU4X"] pub fn double_width_slots (self) -> bool { self . 0 & Self :: DOUBLE_SLOTS_FLAG != 0 } # [doc = " There is no simple casefolding, even if there is a simple lowercase mapping"] pub fn no_simple_case_folding (self) -> bool { self . 0 & Self :: NO_SIMPLE_CASE_FOLDING_FLAG != 0 } # [doc = " The delta stored in the `Delta` slot is negative"] pub fn negative_delta (self) -> bool { self . 0 & Self :: NEGATIVE_DELTA_FLAG != 0 } # [doc = " If the character is case sensitive"] pub fn is_sensitive (self) -> bool { self . 0 & Self :: SENSITIVE_FLAG != 0 } # [doc = " If the character has conditional special casing"] pub fn has_conditional_special (self) -> bool { self . 0 & Self :: CONDITIONAL_SPECIAL_FLAG != 0 } # [doc = " If the character has conditional case folding"] pub fn has_conditional_fold (self) -> bool { self . 0 & Self :: CONDITIONAL_FOLD_FLAG != 0 } # [doc = " The dot type of the character"] pub fn dot_type (self) -> DotType { DotType :: from_masked_bits ((u16 :: from (self . 0 >> Self :: DOT_SHIFT)) & DotType :: DOT_MASK) } }
};
}
