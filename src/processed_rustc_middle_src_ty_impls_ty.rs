// SRC: ../rust/compiler/rustc_middle/src/ty/impls_ty.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=4 | LINES=11 */
// This module contains `HashStable` implementations for various data types
// from `crate::rustc_middle::ty` in no particular order.

use std::cell::RefCell;
use std::ptr;

use crate::rustc_data_structures::fingerprint::Fingerprint;
use crate::rustc_data_structures::fx::FxHashMap;
use crate::rustc_data_structures::stable_hasher::{
    HashStable, HashingControls, StableHasher, ToStableHashKey,
};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=5 */
use rustc_query_system::ich::StableHashingContext;
use tracing::trace;

use crate::middle::region;
use crate::{mir, ty};
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=hash_stable | COMPLEXITY=11 | LINES=28 */

impl<'a, 'tcx, H, T> HashStable<StableHashingContext<'a>> for &'tcx ty::list::RawList<H, T>
where
    T: HashStable<StableHashingContext<'a>>,
{
    fn hash_stable(&self, hcx: &mut StableHashingContext<'a>, hasher: &mut StableHasher) {
        thread_local! {
            static CACHE: RefCell<FxHashMap<(*const (), HashingControls), Fingerprint>> =
                RefCell::new(Default::default());
        }

        let hash = CACHE.with(|cache| {
            let key = (ptr::from_ref(*self).cast::<()>(), hcx.hashing_controls());
            if let Some(&hash) = cache.borrow().get(&key) {
                return hash;
            }

            let mut hasher = StableHasher::new();
            self[..].hash_stable(hcx, &mut hasher);

            let hash: Fingerprint = hasher.finish();
            cache.borrow_mut().insert(key, hash);
            hash
        });

        hash.hash_stable(hcx, hasher);
    }
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=to_stable_hash_key | COMPLEXITY=5 | LINES=15 */

impl<'a, 'tcx, H, T> ToStableHashKey<StableHashingContext<'a>> for &'tcx ty::list::RawList<H, T>
where
    T: HashStable<StableHashingContext<'a>>,
{
    type KeyType = Fingerprint;

    #[inline]
    fn to_stable_hash_key(&self, hcx: &StableHashingContext<'a>) -> Fingerprint {
        let mut hasher = StableHasher::new();
        let mut hcx: StableHashingContext<'a> = hcx.clone();
        self.hash_stable(&mut hcx, &mut hasher);
        hasher.finish()
    }
}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=hash_stable | COMPLEXITY=5 | LINES=6 */

impl<'a, 'tcx> HashStable<StableHashingContext<'a>> for ty::GenericArg<'tcx> {
    fn hash_stable(&self, hcx: &mut StableHashingContext<'a>, hasher: &mut StableHasher) {
        self.kind().hash_stable(hcx, hasher);
    }
}
/* AST_META: AST_ID=6 | TYPE=FUNCTION | NAME=hash_stable | COMPLEXITY=7 | LINES=11 */

// AllocIds get resolved to whatever they point to (to be stable)
impl<'a> HashStable<StableHashingContext<'a>> for mir::interpret::AllocId {
    fn hash_stable(&self, hcx: &mut StableHashingContext<'a>, hasher: &mut StableHasher) {
        ty::tls::with_opt(|tcx| {
            trace!("hashing {:?}", *self);
            let tcx = tcx.expect("can't hash AllocIds during hir lowering");
            tcx.try_get_global_alloc(*self).hash_stable(hcx, hasher);
        });
    }
}
/* AST_META: AST_ID=7 | TYPE=FUNCTION | NAME=hash_stable | COMPLEXITY=5 | LINES=6 */

impl<'a> HashStable<StableHashingContext<'a>> for mir::interpret::CtfeProvenance {
    fn hash_stable(&self, hcx: &mut StableHashingContext<'a>, hasher: &mut StableHasher) {
        self.into_parts().hash_stable(hcx, hasher);
    }
}
/* AST_META: AST_ID=8 | TYPE=FUNCTION | NAME=to_stable_hash_key | COMPLEXITY=5 | LINES=9 */

impl<'a> ToStableHashKey<StableHashingContext<'a>> for region::Scope {
    type KeyType = region::Scope;

    #[inline]
    fn to_stable_hash_key(&self, _: &StableHashingContext<'a>) -> region::Scope {
        *self
    }
}