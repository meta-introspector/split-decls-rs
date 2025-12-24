use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// OID for ECDSA with SHA-512 digests.
///
/// ```text
/// ecdsa-with-SHA512 OBJECT IDENTIFIER ::= { iso(1) member-body(2)
///      us(840) ansi-X9-62(10045) signatures(4) ecdsa-with-SHA2(3) 4 }
/// ```
#[cfg(feature = "digest")]
pub const ECDSA_SHA512_OID: ObjectIdentifier = ObjectIdentifier::new_unwrap(
    "1.2.840.10045.4.3.4",
);
