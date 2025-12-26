use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Internal implementation of `HMAC_DRBG` as described in NIST SP800-90A.
///
/// <https://csrc.nist.gov/publications/detail/sp/800-90a/rev-1/final>
///
/// This is a HMAC-based deterministic random bit generator used compute a
/// deterministic ephemeral scalar `k`.
pub struct HmacDrbg<D>
where
    D: EagerHash,
{
    /// HMAC key `K` (see RFC 6979 Section 3.2.c)
    k: HmacReset<D>,
    /// Chaining value `V` (see RFC 6979 Section 3.2.c)
    v: Array<u8, <D::Core as OutputSizeUser>::OutputSize>,
}
