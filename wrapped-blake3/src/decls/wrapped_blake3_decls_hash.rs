use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The default hash function.
///
/// For an incremental version that accepts multiple writes, see [`Hasher::new`],
/// [`Hasher::update`], and [`Hasher::finalize`]. These two lines are equivalent:
///
/// ```
/// let hash = blake3::hash(b"foo");
/// # let hash1 = hash;
///
/// let hash = blake3::Hasher::new().update(b"foo").finalize();
/// # let hash2 = hash;
/// # assert_eq!(hash1, hash2);
/// ```
///
/// For output sizes other than 32 bytes, see [`Hasher::finalize_xof`] and
/// [`OutputReader`].
///
/// This function is always single-threaded. For multithreading support, see
/// [`Hasher::update_rayon`](struct.Hasher.html#method.update_rayon).
pub fn hash(input: &[u8]) -> Hash {
    hash_all_at_once::<join::SerialJoin>(input, IV, 0).root_hash()
}
