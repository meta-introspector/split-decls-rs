// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_data_structures/src/svh.rs
// Error: expected square brackets
// Problematic line: line 15

use crate::fingerprint::Fingerprint;
use crate::stable_hasher;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Encodable_NoContext, Decodable_NoContext, Hash)]
pub struct Svh {
    hash: Fingerprint,
}
