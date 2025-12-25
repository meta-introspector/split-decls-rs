use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<'a, K: 'a + Eq + Hash, V: 'a, S: BuildHasher + Clone, Q> BitOr<&Q> for &'a DashMap<K, V, S>
where
    Q: Hash + Equivalent<K> + ?Sized,
{
    type Output = RefMut<'a, K, V>;
    fn bitor(self, key: &Q) -> Self::Output {
        self.get_mut(key).unwrap()
    }
}
