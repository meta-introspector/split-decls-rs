// Generated macro for Hkdf (struct)
macro_rules! Depcrate_hkdfHkdf {
() => {
// Module: crate::hkdf
// Provides: {"Hkdf"}
// Dependencies: {}
# [doc = " HKDF for any of the implemented hash functions. The aliases [`HkdfSha256`]"] # [doc = " and [`HkdfSha512`] are provided for the most common cases."] pub struct Hkdf < MD : digest :: Algorithm > (PhantomData < MD >) ;
};
}
