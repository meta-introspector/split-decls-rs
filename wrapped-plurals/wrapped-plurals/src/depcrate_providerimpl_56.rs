// Generated macro for impl_56 (impl)
macro_rules! Depcrate_providerimpl_56 {
() => {
// Module: crate::provider
// Provides: {"impl_56"}
// Dependencies: {}
unsafe impl < V > VarULE for PluralElementsPackedULE < V > where V : VarULE + ? Sized , { fn validate_bytes (bytes : & [u8]) -> Result < () , UleError > { let unpacked_bytes = Self :: unpack_bytes (bytes) . ok_or_else (| | UleError :: length :: < Self > (bytes . len ())) ? ; if unpacked_bytes . lead_byte & 0x70 != 0 { return Err (UleError :: parse :: < Self > ()) ; } V :: validate_bytes (unpacked_bytes . v_bytes) ? ; if let Some (specials_bytes) = unpacked_bytes . specials_bytes { PluralElementsTupleSliceVarULE :: < V > :: validate_bytes (specials_bytes) ? ; } Ok (()) } unsafe fn from_bytes_unchecked (bytes : & [u8]) -> & Self { core :: mem :: transmute (bytes) } }
};
}
