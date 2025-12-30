// Generated macro for ExceptionBits (struct)
macro_rules! Depcrate_provider_exception_helpersExceptionBits {
() => {
// Module: crate::provider::exception_helpers
// Provides: {"ExceptionBits"}
// Dependencies: {}
# [doc = " A bunch of bits associated with each exception."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] # [derive (Copy , Clone , PartialEq , Eq , Debug , Default)] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize))] pub struct ExceptionBits { # [doc = " Whether or not the slots are double-width."] # [doc = ""] # [doc = " Unused in ICU4X"] pub double_width_slots : bool , # [doc = " There is no simple casefolding, even if there is a simple lowercase mapping"] pub no_simple_case_folding : bool , # [doc = " The delta stored in the `Delta` slot is negative"] pub negative_delta : bool , # [doc = " If the character is case sensitive"] pub is_sensitive : bool , # [doc = " The dot type of the character"] pub dot_type : DotType , # [doc = " If the character has conditional special casing"] pub has_conditional_special : bool , # [doc = " If the character has conditional case folding"] pub has_conditional_fold : bool , }
};
}
