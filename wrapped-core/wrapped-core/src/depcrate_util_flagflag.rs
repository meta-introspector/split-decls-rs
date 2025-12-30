// Generated macro for Flag (struct)
macro_rules! Depcrate_util_flagFlag {
() => {
// Module: crate::util::flag
// Provides: {"Flag"}
// Dependencies: {}
# [doc = " A meta-item that can be present as a word - with no value - or absent."] # [doc = ""] # [doc = " # Defaulting"] # [doc = " Like `Option`, `Flag` does not require `#[darling(default)]` to be optional."] # [doc = " If the caller does not include the property, then an absent `Flag` will be included"] # [doc = " in the receiver struct."] # [doc = ""] # [doc = " # Spans"] # [doc = " `Flag` keeps the span where its word was seen."] # [doc = " This enables attaching custom error messages to the word, such as in the case of two"] # [doc = " conflicting flags being present."] # [doc = ""] # [doc = " # Example"] # [doc = " ```ignore"] # [doc = " #[derive(FromMeta)]"] # [doc = " #[darling(and_then = Self::not_both)]"] # [doc = " struct Demo {"] # [doc = "     flag_a: Flag,"] # [doc = "     flag_b: Flag,"] # [doc = " }"] # [doc = ""] # [doc = " impl Demo {"] # [doc = "     fn not_both(self) -> Result<Self> {"] # [doc = "         if self.flag_a.is_present() && self.flag_b.is_present() {"] # [doc = "             Err(Error::custom(\"Cannot set flag_a and flag_b\").with_span(&self.flag_b.span()))"] # [doc = "         } else {"] # [doc = "             Ok(self)"] # [doc = "         }"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " The above struct would then produce the following error."] # [doc = ""] # [doc = " ```ignore"] # [doc = " #[example(flag_a, flag_b)]"] # [doc = " //                ^^^^^^ Cannot set flag_a and flag_b"] # [doc = " ```"] # [derive (Debug , Clone , Copy , Default)] pub struct Flag (Option < Span >) ;
};
}
