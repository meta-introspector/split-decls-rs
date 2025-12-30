// Generated macro for impl_75 (impl)
macro_rules! Depcrate_frontend_zerovecimpl_75 {
() => {
// Module: crate::frontend::zerovec
// Provides: {"impl_75"}
// Dependencies: {}
# [doc = " Implement `VarULE` for `Pattern<SinglePlaceholder, str>`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Safety checklist for `ULE`:"] # [doc = ""] # [doc = " 1. `Pattern<B>` does not include any uninitialized or padding bytes."] # [doc = " 2. `Pattern<B>` is aligned to 1 byte."] # [doc = " 3. The implementation of `validate_bytes()` returns an error"] # [doc = "    if any byte is not valid."] # [doc = " 4. The implementation of `validate_bytes()` returns an error"] # [doc = "    if the slice cannot be used to build a `Pattern<B>` in its entirety."] # [doc = " 5. The implementation of `from_bytes_unchecked()` returns a reference to the same data."] # [doc = " 6. `parse_bytes()` is equivalent to `validate_bytes()` followed by `from_bytes_unchecked()`."] # [doc = " 7. `Pattern<B>` byte equality is semantic equality."] unsafe impl < B , S : ? Sized + VarULE > VarULE for Pattern < B > where B : PatternBackend < Store = S > , { fn validate_bytes (bytes : & [u8]) -> Result < () , UleError > { let store = S :: parse_bytes (bytes) ? ; B :: validate_store (store) . map_err (| _ | UleError :: parse :: < Self > ()) } unsafe fn from_bytes_unchecked (bytes : & [u8]) -> & Self { let store = S :: from_bytes_unchecked (bytes) ; Self :: from_ref_store_unchecked (store) } }
};
}
