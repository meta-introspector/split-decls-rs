use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<K: Eq + Hash + fmt::Debug, V: fmt::Debug, S: BuildHasher + Clone> fmt::Debug
    for DashMap<K, V, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut pmap = f.debug_map();
        for r in self {
            let (k, v) = r.pair();
            pmap.entry(k, v);
        }
        pmap.finish()
    }
}
