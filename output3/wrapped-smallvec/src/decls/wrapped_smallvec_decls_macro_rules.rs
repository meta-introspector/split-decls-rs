use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[macro_export]
macro_rules! smallvec_inline {
    (@ one $x:expr) => {
        1usize
    };
    ($elem:expr; $n:expr) => {
        { $crate::SmallVec::< _, $n >::from_buf([$elem; $n]) }
    };
    ($($x:expr),+ $(,)?) => {
        { const N : usize = 0usize $(+ $crate::smallvec_inline!(@ one $x))*;
        $crate::SmallVec::< _, N >::from_buf([$($x,)*]) }
    };
}
