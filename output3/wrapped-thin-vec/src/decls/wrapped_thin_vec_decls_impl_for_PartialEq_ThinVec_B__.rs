use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<A, B> PartialEq<ThinVec<B>> for ThinVec<A>
where
    A: PartialEq<B>,
{
    #[inline]
    fn eq(&self, other: &ThinVec<B>) -> bool {
        self[..] == other[..]
    }
}
