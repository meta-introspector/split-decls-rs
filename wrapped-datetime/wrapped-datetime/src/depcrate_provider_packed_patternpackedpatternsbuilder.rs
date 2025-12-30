// Generated macro for PackedPatternsBuilder (struct)
macro_rules! Depcrate_provider_packed_patternPackedPatternsBuilder {
() => {
// Module: crate::provider::packed_pattern
// Provides: {"PackedPatternsBuilder"}
// Dependencies: {}
# [doc = " A builder for a [`PackedPatterns`]."] # [derive (Debug , Clone , PartialEq , Eq)] pub struct PackedPatternsBuilder < 'a > { # [doc = " Patterns always available."] pub standard : LengthPluralElements < Pattern < 'a > > , # [doc = " Patterns for variant 0. If `None`, falls back to standard."] pub variant0 : Option < LengthPluralElements < Pattern < 'a > > > , # [doc = " Patterns for variant 1. If `None`, falls back to standard."] pub variant1 : Option < LengthPluralElements < Pattern < 'a > > > , }
};
}
