// SRC: ../rust/compiler/rustc_data_structures/src/packed.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=4 */
use std::cmp::Ordering;
use std::fmt;

use crate::rustc_serialize::{Decodable, Decoder, Encodable, Encoder};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */

use crate::stable_hasher::{HashStable, StableHasher};
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=Pu128(pub | COMPLEXITY=5 | LINES=13 */

/// A packed 128-bit integer. Useful for reducing the size of structures in
/// some cases.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(packed(8))]
pub struct Pu128(pub u128);

impl Pu128 {
    #[inline]
    pub fn get(self) -> u128 {
        self.0
    }
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=from | COMPLEXITY=5 | LINES=7 */

impl From<Pu128> for u128 {
    #[inline]
    fn from(value: Pu128) -> Self {
        value.get()
    }
}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=from | COMPLEXITY=5 | LINES=7 */

impl From<u128> for Pu128 {
    #[inline]
    fn from(value: u128) -> Self {
        Self(value)
    }
}
/* AST_META: AST_ID=6 | TYPE=FUNCTION | NAME=eq | COMPLEXITY=6 | LINES=7 */

impl PartialEq<u128> for Pu128 {
    #[inline]
    fn eq(&self, other: &u128) -> bool {
        ({ self.0 }) == *other
    }
}
/* AST_META: AST_ID=7 | TYPE=FUNCTION | NAME=partial_cmp | COMPLEXITY=6 | LINES=7 */

impl PartialOrd<u128> for Pu128 {
    #[inline]
    fn partial_cmp(&self, other: &u128) -> Option<Ordering> {
        { self.0 }.partial_cmp(other)
    }
}
/* AST_META: AST_ID=8 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=6 | LINES=7 */

impl fmt::Display for Pu128 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        { self.0 }.fmt(f)
    }
}
/* AST_META: AST_ID=9 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=6 | LINES=7 */

impl fmt::UpperHex for Pu128 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        { self.0 }.fmt(f)
    }
}
/* AST_META: AST_ID=10 | TYPE=FUNCTION | NAME=hash_stable | COMPLEXITY=6 | LINES=7 */

impl<CTX> HashStable<CTX> for Pu128 {
    #[inline]
    fn hash_stable(&self, ctx: &mut CTX, hasher: &mut StableHasher) {
        { self.0 }.hash_stable(ctx, hasher)
    }
}
/* AST_META: AST_ID=11 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=6 | LINES=7 */

impl<S: Encoder> Encodable<S> for Pu128 {
    #[inline]
    fn encode(&self, s: &mut S) {
        { self.0 }.encode(s);
    }
}
/* AST_META: AST_ID=12 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=7 */

impl<D: Decoder> Decodable<D> for Pu128 {
    #[inline]
    fn decode(d: &mut D) -> Self {
        Self(u128::decode(d))
    }
}