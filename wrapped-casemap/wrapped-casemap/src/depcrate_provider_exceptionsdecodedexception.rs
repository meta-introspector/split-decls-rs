// Generated macro for DecodedException (struct)
macro_rules! Depcrate_provider_exceptionsDecodedException {
() => {
// Module: crate::provider::exceptions
// Provides: {"DecodedException"}
// Dependencies: {}
# [doc = " A decoded [`Exception`] type, with all of the data parsed out into"] # [doc = " separate fields."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize))] # [derive (Debug , Clone , PartialEq , Eq , Default)] pub struct DecodedException < 'a > { # [doc = " The various bit-based data associated with this exception"] pub bits : ExceptionBits , # [doc = " Lowercase mapping"] pub lowercase : Option < char > , # [doc = " Case folding"] pub casefold : Option < char > , # [doc = " Uppercase mapping"] pub uppercase : Option < char > , # [doc = " Titlecase mapping"] pub titlecase : Option < char > , # [doc = " The simple casefold delta. Its sign is stored in bits.negative_delta"] pub simple_case_delta : Option < u32 > , # [doc = " Closure mappings"] pub closure : Option < Cow < 'a , str > > , # [doc = " The four full-mappings strings, indexed by MappingKind u8 value"] pub full : Option < [Cow < 'a , str > ; 4] > , }
};
}
