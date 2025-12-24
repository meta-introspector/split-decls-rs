use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An output of the default size, 32 bytes, which provides constant-time
/// equality checking.
///
/// `Hash` implements [`From`] and [`Into`] for `[u8; 32]`, and it provides
/// [`from_bytes`] and [`as_bytes`] for explicit conversions between itself and
/// `[u8; 32]`. However, byte arrays and slices don't provide constant-time
/// equality checking, which is often a security requirement in software that
/// handles private data. `Hash` doesn't implement [`Deref`] or [`AsRef`], to
/// avoid situations where a type conversion happens implicitly and the
/// constant-time property is accidentally lost.
///
/// `Hash` provides the [`to_hex`] and [`from_hex`] methods for converting to
/// and from hexadecimal. It also implements [`Display`] and [`FromStr`].
///
/// [`From`]: https://doc.rust-lang.org/std/convert/trait.From.html
/// [`Into`]: https://doc.rust-lang.org/std/convert/trait.Into.html
/// [`as_bytes`]: #method.as_bytes
/// [`from_bytes`]: #method.from_bytes
/// [`Deref`]: https://doc.rust-lang.org/stable/std/ops/trait.Deref.html
/// [`AsRef`]: https://doc.rust-lang.org/std/convert/trait.AsRef.html
/// [`to_hex`]: #method.to_hex
/// [`from_hex`]: #method.from_hex
/// [`Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
/// [`FromStr`]: https://doc.rust-lang.org/std/str/trait.FromStr.html
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Hash, Eq)]
pub struct Hash([u8; OUT_LEN]);
