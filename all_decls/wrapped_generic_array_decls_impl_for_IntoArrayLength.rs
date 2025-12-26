use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<N> IntoArrayLength for N
where
    N: ArrayLength,
{
    type ArrayLength = Self;
}
