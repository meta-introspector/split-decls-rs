// Generated macro for impl_316 (impl)
macro_rules! Depcrate_dimension_provider_units_pattern_keyimpl_316 {
() => {
// Module: crate::dimension::provider::units::pattern_key
// Provides: {"impl_316"}
// Dependencies: {}
unsafe impl ULE for PatternKeyULE { fn validate_bytes (bytes : & [u8]) -> Result < () , zerovec :: ule :: UleError > { for & byte in bytes . iter () { if (byte & 0b1100_0000) == 0b1100_0000 { return Err (UleError :: parse :: < Self > ()) ; } if (byte & 0b1100_0000) == 0b1000_0000 { if (byte & 0b0010_0000) == 0 { return Err (UleError :: parse :: < Self > ()) ; } if (byte & 0b0000_1000) != 0 { return Err (UleError :: parse :: < Self > ()) ; } if (byte & 0b0000_0100) != 0 && (byte & 0b0000_0010) != 0 { return Err (UleError :: parse :: < Self > ()) ; } } } Ok (()) } }
};
}
