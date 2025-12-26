use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T: Internable + ?Sized> InternStorage<T> {
    fn get(&self) -> &InternMap<T> {
        self.map
            .get_or_init(|| DashMap::<Arc<T>, (), BuildHasherDefault<FxHasher>>::default())
    }
}
