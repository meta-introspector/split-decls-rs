use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The keyed hash function.
///
/// This is suitable for use as a message authentication code, for example to
/// replace an HMAC instance. In that use case, the constant-time equality
/// checking provided by [`Hash`](struct.Hash.html) is almost always a security
/// requirement, and callers need to be careful not to compare MACs as raw
/// bytes.
///
/// For an incremental version that accepts multiple writes, see [`Hasher::new_keyed`],
/// [`Hasher::update`], and [`Hasher::finalize`]. These two lines are equivalent:
///
/// ```
/// # const KEY: &[u8; 32] = &[0; 32];
/// let mac = blake3::keyed_hash(KEY, b"foo");
/// # let mac1 = mac;
///
/// let mac = blake3::Hasher::new_keyed(KEY).update(b"foo").finalize();
/// # let mac2 = mac;
/// # assert_eq!(mac1, mac2);
/// ```
///
/// For output sizes other than 32 bytes, see [`Hasher::finalize_xof`], and [`OutputReader`].
///
/// This function is always single-threaded. For multithreading support, see
/// [`Hasher::update_rayon`](struct.Hasher.html#method.update_rayon).
pub fn keyed_hash(key: &[u8; KEY_LEN], input: &[u8]) -> Hash {
    let key_words = platform::words_from_le_bytes_32(key);
    hash_all_at_once::<join::SerialJoin>(input, &key_words, KEYED_HASH).root_hash()
}
