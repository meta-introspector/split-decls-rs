use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Deterministically generate ephemeral scalar `k` by writing it into the provided output buffer.
///
/// This is an API which accepts dynamically sized inputs intended for use cases where the sizes
/// are determined at runtime, such as the legacy Digital Signature Algorithm (DSA).
///
/// Accepts the following parameters and inputs:
///
/// - `x`: secret key
/// - `q`: field modulus
/// - `h`: hash/digest of input message: must be reduced modulo `q` in advance
/// - `data`: additional associated data, e.g. CSRNG output used as added entropy
#[inline]
pub fn generate_k_mut<D>(x: &[u8], q: &[u8], h: &[u8], data: &[u8], k: &mut [u8])
where
    D: EagerHash,
{
    let k_len = k.len();
    assert_eq!(k_len, x.len());
    assert_eq!(k_len, q.len());
    assert_eq!(k_len, h.len());
    debug_assert!(bool::from(ct::lt(h, q)));
    let q_leading_zeros = ct::leading_zeros(q);
    let q_has_leading_zeros = q_leading_zeros != 0;
    let mut hmac_drbg = HmacDrbg::<D>::new(x, h, data);
    loop {
        hmac_drbg.fill_bytes(k);
        if q_has_leading_zeros {
            ct::rshift(k, q_leading_zeros);
        }
        if (!ct::is_zero(k) & ct::lt(k, q)).into() {
            return;
        }
    }
}
