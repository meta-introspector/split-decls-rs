use serde::{Deserialize, Serialize};
use std::collections::HashMap;
mod sealed {
    use crate::typenum::{IsLess, NonZero, True, Unsigned, U256};
    pub trait BlockSizes {}
    impl<T: Unsigned> BlockSizes for T where Self: IsLess<U256, Output = True> + NonZero {}
}
