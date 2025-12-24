use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Types which can be initialized from key.
pub trait KeyInit: KeySizeUser + Sized {
    /// Create new value from fixed size key.
    fn new(key: &Key<Self>) -> Self;
    /// Check if the key might be considered weak.
    #[inline]
    fn weak_key_test(_key: &Key<Self>) -> Result<(), WeakKeyError> {
        Ok(())
    }
    /// Create new value from fixed size key after checking it for weakness.
    #[inline]
    fn new_checked(key: &Key<Self>) -> Result<Self, WeakKeyError> {
        Self::weak_key_test(key)?;
        Ok(Self::new(key))
    }
    /// Create new value from variable size key.
    #[inline]
    fn new_from_slice(key: &[u8]) -> Result<Self, InvalidLength> {
        <&Key<Self>>::try_from(key).map(Self::new).map_err(|_| InvalidLength)
    }
    /// Generate random key using the operating system's secure RNG.
    #[cfg(feature = "getrandom")]
    #[inline]
    fn generate_key() -> Result<Key<Self>, getrandom::Error> {
        let mut key = Key::<Self>::default();
        getrandom::fill(&mut key)?;
        Ok(key)
    }
    /// Generate random key using the provided [`CryptoRng`].
    #[cfg(feature = "rand_core")]
    #[inline]
    fn generate_key_with_rng<R: CryptoRng + ?Sized>(rng: &mut R) -> Key<Self> {
        let mut key = Key::<Self>::default();
        rng.fill_bytes(&mut key);
        key
    }
    /// Generate random key using the provided [`TryCryptoRng`].
    #[cfg(feature = "rand_core")]
    #[inline]
    fn try_generate_key_with_rng<R: TryCryptoRng + ?Sized>(
        rng: &mut R,
    ) -> Result<Key<Self>, R::Error> {
        let mut key = Key::<Self>::default();
        rng.try_fill_bytes(&mut key)?;
        Ok(key)
    }
}
