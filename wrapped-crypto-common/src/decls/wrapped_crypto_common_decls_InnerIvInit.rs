use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Types which can be initialized from another type and additional initialization
/// vector/nonce.
///
/// Usually used for initializing types from block ciphers.
pub trait InnerIvInit: InnerUser + IvSizeUser + Sized {
    /// Initialize value using `inner` and `iv` array.
    fn inner_iv_init(inner: Self::Inner, iv: &Iv<Self>) -> Self;
    /// Initialize value using `inner` and `iv` slice.
    #[inline]
    fn inner_iv_slice_init(inner: Self::Inner, iv: &[u8]) -> Result<Self, InvalidLength> {
        let iv = <&Iv<Self>>::try_from(iv).map_err(|_| InvalidLength)?;
        Ok(Self::inner_iv_init(inner, iv))
    }
    /// Generate random IV using the operating system's secure RNG.
    #[cfg(feature = "getrandom")]
    #[inline]
    fn generate_iv() -> Result<Iv<Self>, getrandom::Error> {
        let mut iv = Iv::<Self>::default();
        getrandom::fill(&mut iv)?;
        Ok(iv)
    }
    /// Generate random IV using the provided [`CryptoRng`].
    #[cfg(feature = "rand_core")]
    #[inline]
    fn generate_iv_with_rng<R: CryptoRng + ?Sized>(rng: &mut R) -> Iv<Self> {
        let mut iv = Iv::<Self>::default();
        rng.fill_bytes(&mut iv);
        iv
    }
    /// Generate random IV using the provided [`TryCryptoRng`].
    #[cfg(feature = "rand_core")]
    #[inline]
    fn try_generate_iv_with_rng<R: TryCryptoRng + ?Sized>(
        rng: &mut R,
    ) -> Result<Iv<Self>, R::Error> {
        let mut iv = Iv::<Self>::default();
        rng.try_fill_bytes(&mut iv)?;
        Ok(iv)
    }
}
