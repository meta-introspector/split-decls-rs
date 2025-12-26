use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Error indicating that the maximum recursion depth has been reached while calculating [`Arbitrary::size_hint`]()
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct MaxRecursionReached {}
