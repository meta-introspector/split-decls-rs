use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T, S: Strategy<T>> Debug for ArcSwapAny<T, S>
where
    T: Debug + RefCnt,
{
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        formatter.debug_tuple("ArcSwapAny").field(&self.load()).finish()
    }
}
