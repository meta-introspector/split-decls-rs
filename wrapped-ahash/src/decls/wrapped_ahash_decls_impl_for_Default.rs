use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Provides a default [Hasher] with fixed keys.
/// This is typically used in conjunction with [BuildHasherDefault] to create
/// [AHasher]s in order to hash the keys of the map.
///
/// Generally it is preferable to use [RandomState] instead, so that different
/// hashmaps will have different keys. However if fixed keys are desirable this
/// may be used instead.
///
/// # Example
/// ```
/// use std::hash::BuildHasherDefault;
/// use ahash::{AHasher, RandomState};
/// use std::collections::HashMap;
///
/// let mut map: HashMap<i32, i32, BuildHasherDefault<AHasher>> = HashMap::default();
/// map.insert(12, 34);
/// ```
///
/// [BuildHasherDefault]: std::hash::BuildHasherDefault
/// [Hasher]: std::hash::Hasher
/// [HashMap]: std::collections::HashMap
impl Default for AHasher {
    /// Constructs a new [AHasher] with fixed keys.
    /// If `std` is enabled these will be generated upon first invocation.
    /// Otherwise if the `compile-time-rng`feature is enabled these will be generated at compile time.
    /// If neither of these features are available, hardcoded constants will be used.
    ///
    /// Because the values are fixed, different hashers will all hash elements the same way.
    /// This could make hash values predictable, if DOS attacks are a concern. If this behaviour is
    /// not required, it may be preferable to use [RandomState] instead.
    ///
    /// # Examples
    ///
    /// ```
    /// use ahash::AHasher;
    /// use std::hash::Hasher;
    ///
    /// let mut hasher_1 = AHasher::default();
    /// let mut hasher_2 = AHasher::default();
    ///
    /// hasher_1.write_u32(1234);
    /// hasher_2.write_u32(1234);
    ///
    /// assert_eq!(hasher_1.finish(), hasher_2.finish());
    /// ```
    #[inline]
    fn default() -> AHasher {
        RandomState::with_fixed_keys().build_hasher()
    }
}
