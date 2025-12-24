use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An extension of [`core::hash::Hasher`] for 32-bit hashers.
///
/// For hashers that implement this trait, the [`core::hash::Hasher::finish`] method should return a
/// zero-extended version of the result from [`Hasher::finish32`].
///
/// # Contract
///
/// Implementers of this trait must **not** perform any 64-bit (or 128-bit) operation while computing
/// the hash.
///
/// # Examples
///
/// ```
/// use core::hash::Hasher as _;
/// use hash32::{FnvHasher, Hasher as _};
///
/// let mut hasher: FnvHasher = Default::default();
///
/// hasher.write_u32(1989);
/// hasher.write_u8(11);
/// hasher.write_u8(9);
/// hasher.write(b"Huh?");
///
/// println!("Hash is {:x}!", hasher.finish32());
/// ```
pub trait Hasher: core::hash::Hasher {
    /// The equivalent of [`core::hash::Hasher::finish`] for 32-bit hashers.
    ///
    /// This returns the hash directly; [`core::hash::Hasher::finish`] zero-extends the `finish32`
    /// result to 64-bits for compatibility.
    fn finish32(&self) -> u32;
}
