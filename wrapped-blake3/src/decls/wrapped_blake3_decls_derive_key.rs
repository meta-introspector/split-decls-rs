use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The key derivation function.
///
/// Given cryptographic key material of any length and a context string of any
/// length, this function outputs a 32-byte derived subkey. **The context string
/// should be hardcoded, globally unique, and application-specific.** A good
/// default format for such strings is `"[application] [commit timestamp]
/// [purpose]"`, e.g., `"example.com 2019-12-25 16:18:03 session tokens v1"`.
///
/// Key derivation is important when you want to use the same key in multiple
/// algorithms or use cases. Using the same key with different cryptographic
/// algorithms is generally forbidden, and deriving a separate subkey for each
/// use case protects you from bad interactions. Derived keys also mitigate the
/// damage from one part of your application accidentally leaking its key.
///
/// As a rare exception to that general rule, however, it is possible to use
/// `derive_key` itself with key material that you are already using with
/// another algorithm. You might need to do this if you're adding features to
/// an existing application, which does not yet use key derivation internally.
/// However, you still must not share key material with algorithms that forbid
/// key reuse entirely, like a one-time pad. For more on this, see sections 6.2
/// and 7.8 of the [BLAKE3 paper](https://github.com/BLAKE3-team/BLAKE3-specs/blob/master/blake3.pdf).
///
/// Note that BLAKE3 is not a password hash, and **`derive_key` should never be
/// used with passwords.** Instead, use a dedicated password hash like
/// [Argon2]. Password hashes are entirely different from generic hash
/// functions, with opposite design requirements.
///
/// For an incremental version that accepts multiple writes, see [`Hasher::new_derive_key`],
/// [`Hasher::update`], and [`Hasher::finalize`]. These two statements are equivalent:
///
/// ```
/// # const CONTEXT: &str = "example.com 2019-12-25 16:18:03 session tokens v1";
/// let key = blake3::derive_key(CONTEXT, b"key material, not a password");
/// # let key1 = key;
///
/// let key: [u8; 32] = blake3::Hasher::new_derive_key(CONTEXT)
///     .update(b"key material, not a password")
///     .finalize()
///     .into();
/// # let key2 = key;
/// # assert_eq!(key1, key2);
/// ```
///
/// For output sizes other than 32 bytes, see [`Hasher::finalize_xof`], and [`OutputReader`].
///
/// This function is always single-threaded. For multithreading support, see
/// [`Hasher::update_rayon`](struct.Hasher.html#method.update_rayon).
///
/// [Argon2]: https://en.wikipedia.org/wiki/Argon2
pub fn derive_key(context: &str, key_material: &[u8]) -> [u8; OUT_LEN] {
    let context_key = hazmat::hash_derive_key_context(context);
    let context_key_words = platform::words_from_le_bytes_32(&context_key);
    hash_all_at_once::<join::SerialJoin>(key_material, &context_key_words, DERIVE_KEY_MATERIAL)
        .root_hash()
        .0
}
