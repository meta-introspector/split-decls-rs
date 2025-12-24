use serde::{Deserialize, Serialize};
use std::collections::HashMap;
macro_rules! commutative_op {
    ($t:ty) => {
        impl ops::Add < ByteSize > for $t { type Output = ByteSize; #[inline(always)] fn
        add(self, rhs : ByteSize) -> ByteSize { ByteSize(rhs.0 + (self as u64)) } } impl
        ops::Mul < ByteSize > for $t { type Output = ByteSize; #[inline(always)] fn
        mul(self, rhs : ByteSize) -> ByteSize { ByteSize(rhs.0 * (self as u64)) } }
    };
}
