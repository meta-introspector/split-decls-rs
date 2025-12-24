use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// ECDSA signature (fixed-size, a.k.a. [IEEE P1363]). Generic over elliptic curve types.
///
/// Serialized as fixed-sized big endian scalar values with no added framing:
///
/// - `r`: field element size for the given curve, big-endian
/// - `s`: field element size for the given curve, big-endian
///
/// Both `r` and `s` MUST be non-zero.
///
/// For example, in a curve with a 256-bit modulus like NIST P-256 or
/// secp256k1, `r` and `s` will both be 32-bytes and serialized as big endian,
/// resulting in a signature with a total of 64-bytes.
///
/// ASN.1 DER-encoded signatures also supported via the
/// [`Signature::from_der`] and [`Signature::to_der`] methods.
///
/// # `serde` support
///
/// When the `serde` feature of this crate is enabled, it provides support for
/// serializing and deserializing ECDSA signatures using the `Serialize` and
/// `Deserialize` traits.
///
/// The serialization uses a hexadecimal encoding when used with
/// "human readable" text formats, and a binary encoding otherwise.
///
/// [IEEE P1363]: https://en.wikipedia.org/wiki/IEEE_P1363
#[derive(Clone, Eq, PartialEq)]
pub struct Signature<C: EcdsaCurve> {
    r: ScalarValue<C>,
    s: ScalarValue<C>,
}
