// Generated macro for impl_57 (impl)
macro_rules! Depcrate_provider_dataimpl_57 {
() => {
// Module: crate::provider::data
// Provides: {"impl_57"}
// Dependencies: {}
# [doc = " # Safety"] # [doc = ""] # [doc = " Safety checklist for `ULE`:"] # [doc = ""] # [doc = " 1. The type *must not* include any uninitialized or padding bytes: repr(transparent)"] # [doc = "    wrapper around ULE type"] # [doc = " 2. The type must have an alignment of 1 byte: repr(transparent) wrapper around ULE type"] # [doc = " 3. The impl of [`ULE::validate_bytes()`] *must* return an error if the given byte slice"] # [doc = "    would not represent a valid slice of this type: It does"] # [doc = " 4. The impl of [`ULE::validate_bytes()`] *must* return an error if the given byte slice"] # [doc = "    cannot be used in its entirety (if its length is not a multiple of `size_of::<Self>()`):"] # [doc = "    it does, due to the RawBytesULE parse call"] # [doc = " 5. All other methods *must* be left with their default impl, or else implemented according to"] # [doc = "    their respective safety guidelines: They have been"] # [doc = " 6. The equality invariant is satisfied"] unsafe impl ULE for CaseMapDataULE { fn validate_bytes (bytes : & [u8]) -> Result < () , UleError > { let sixteens = RawBytesULE :: < 2 > :: parse_bytes_to_slice (bytes) ? ; for sixteen in sixteens { let sixteen = sixteen . as_unsigned_int () ; if sixteen & Self :: EXCEPTION_BIT == 0 { if sixteen & Self :: CASE_TYPE_BITS == 0 { if sixteen >> Self :: DELTA_SHIFT != 0 { return Err (UleError :: parse :: < Self > ()) ; } } } } Ok (()) } }
};
}
