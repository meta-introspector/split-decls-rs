use serde::{Deserialize, Serialize};
use std::collections::HashMap;
mod sealed {
    pub trait Integer {}
    macro_rules! impl_integer {
        ($($t:ty)*) => {
            $(impl Integer for $t {})*
        };
    }
    impl_integer! {
        u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize
    }
    pub trait ExtendTargetSealed<T> {
        fn extend(self) -> T;
    }
    pub trait TruncateTargetSealed<T> {
        fn truncate(self) -> T;
    }
}
