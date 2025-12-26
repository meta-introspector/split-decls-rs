use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<CTX: HashStableContext> HashStable<CTX> for OwnerId {
    #[inline]
    fn hash_stable(&self, hcx: &mut CTX, hasher: &mut StableHasher) {
        self.to_stable_hash_key(hcx).hash_stable(hcx, hasher);
    }
}
