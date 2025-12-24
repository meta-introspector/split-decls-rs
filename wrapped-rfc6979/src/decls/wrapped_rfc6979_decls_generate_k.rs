use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Deterministically generate ephemeral scalar `k`.
///
/// Accepts the following parameters and inputs:
///
/// - `x`: secret key
/// - `q`: field modulus
/// - `h`: hash/digest of input message: must be reduced modulo `q` in advance
/// - `data`: additional associated data, e.g. CSRNG output used as added entropy
#[inline]
pub fn generate_k<D, N>(
    x: &Array<u8, N>,
    q: &Array<u8, N>,
    h: &Array<u8, N>,
    data: &[u8],
) -> Array<u8, N>
where
    D: EagerHash,
    N: ArraySize,
{
    let mut k = Array::default();
    generate_k_mut::<D>(x, q, h, data, &mut k);
    k
}
