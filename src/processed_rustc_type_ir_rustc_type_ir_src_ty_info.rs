// SRC: ../rust/compiler/rustc_type_ir/src/ty_info.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=6 */
use std::ops::Deref;

#[cfg(feature = "nightly")]
use crate::rustc_data_structures::fingerprint::Fingerprint;
#[cfg(feature = "nightly")]
use crate::rustc_data_structures::stable_hasher::{HashStable, StableHasher};
/* AST_META: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */

use crate::{DebruijnIndex, TypeFlags};
/* AST_META: AST_ID=4 | TYPE=STRUCT | NAME=WithCachedTypeInfo | COMPLEXITY=15 | LINES=44 */

/// A helper type that you can wrap round your own type in order to automatically
/// cache the stable hash, type flags and debruijn index on creation and
/// not recompute it whenever the information is needed.
/// This is only done in incremental mode. You can also opt out of caching by using
/// StableHash::ZERO for the hash, in which case the hash gets computed each time.
/// This is useful if you have values that you intern but never (can?) use for stable
/// hashing.
#[derive(Copy, Clone)]
pub struct WithCachedTypeInfo<T> {
    pub internee: T,

    #[cfg(feature = "nightly")]
    pub stable_hash: Fingerprint,

    /// This field provides fast access to information that is also contained
    /// in `kind`.
    ///
    /// This field shouldn't be used directly and may be removed in the future.
    /// Use `Ty::flags()` instead.
    pub flags: TypeFlags,

    /// This field provides fast access to information that is also contained
    /// in `kind`.
    ///
    /// This is a kind of confusing thing: it stores the smallest
    /// binder such that
    ///
    /// (a) the binder itself captures nothing but
    /// (b) all the late-bound things within the type are captured
    ///     by some sub-binder.
    ///
    /// So, for a type without any late-bound things, like `u32`, this
    /// will be *innermost*, because that is the innermost binder that
    /// captures nothing. But for a type `&'D u32`, where `'D` is a
    /// late-bound region with De Bruijn index `D`, this would be `D + 1`
    /// -- the binder itself does not capture `D`, but `D` is captured
    /// by an inner binder.
    ///
    /// We call this concept an "exclusive" binder `D` because all
    /// De Bruijn indices within the type are contained within `0..D`
    /// (exclusive).
    pub outer_exclusive_binder: DebruijnIndex,
}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=eq | COMPLEXITY=5 | LINES=7 */

impl<T: PartialEq> PartialEq for WithCachedTypeInfo<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.internee.eq(&other.internee)
    }
}
/* AST_META: AST_ID=6 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=2 */

impl<T: Eq> Eq for WithCachedTypeInfo<T> {}
/* AST_META: AST_ID=7 | TYPE=FUNCTION | NAME=partial_cmp | COMPLEXITY=5 | LINES=6 */

impl<T: Ord> PartialOrd for WithCachedTypeInfo<T> {
    fn partial_cmp(&self, other: &WithCachedTypeInfo<T>) -> Option<Ordering> {
        Some(self.internee.cmp(&other.internee))
    }
}
/* AST_META: AST_ID=8 | TYPE=FUNCTION | NAME=cmp | COMPLEXITY=5 | LINES=6 */

impl<T: Ord> Ord for WithCachedTypeInfo<T> {
    fn cmp(&self, other: &WithCachedTypeInfo<T>) -> Ordering {
        self.internee.cmp(&other.internee)
    }
}
/* AST_META: AST_ID=9 | TYPE=FUNCTION | NAME=deref | COMPLEXITY=5 | LINES=9 */

impl<T> Deref for WithCachedTypeInfo<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &self.internee
    }
}
/* AST_META: AST_ID=10 | TYPE=FUNCTION | NAME=hash | COMPLEXITY=8 | LINES=12 */

impl<T: Hash> Hash for WithCachedTypeInfo<T> {
    #[inline]
    fn hash<H: Hasher>(&self, s: &mut H) {
        #[cfg(feature = "nightly")]
        if self.stable_hash != Fingerprint::ZERO {
            return self.stable_hash.hash(s);
        }

        self.internee.hash(s)
    }
}
/* AST_META: AST_ID=11 | TYPE=FUNCTION | NAME=hash_stable | COMPLEXITY=20 | LINES=28 */

#[cfg(feature = "nightly")]
impl<T: HashStable<CTX>, CTX> HashStable<CTX> for WithCachedTypeInfo<T> {
    fn hash_stable(&self, hcx: &mut CTX, hasher: &mut StableHasher) {
        if self.stable_hash == Fingerprint::ZERO || cfg!(debug_assertions) {
            // No cached hash available. This can only mean that incremental is disabled.
            // We don't cache stable hashes in non-incremental mode, because they are used
            // so rarely that the performance actually suffers.

            // We need to build the hash as if we cached it and then hash that hash, as
            // otherwise the hashes will differ between cached and non-cached mode.
            let stable_hash: Fingerprint = {
                let mut hasher = StableHasher::new();
                self.internee.hash_stable(hcx, &mut hasher);
                hasher.finish()
            };
            if cfg!(debug_assertions) && self.stable_hash != Fingerprint::ZERO {
                assert_eq!(
                    stable_hash, self.stable_hash,
                    "cached stable hash does not match freshly computed stable hash"
                );
            }
            stable_hash.hash_stable(hcx, hasher);
        } else {
            self.stable_hash.hash_stable(hcx, hasher);
        }
    }
}