use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Information about one scalar component of a Rust type.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "nightly", derive(HashStable_Generic))]
pub enum Scalar {
    Initialized {
        value: Primitive,
        valid_range: WrappingRange,
    },
    Union {
        /// Even for unions, we need to use the correct registers for the kind of
        /// values inside the union, so we keep the `Primitive` type around. We
        /// also use it to compute the size of the scalar.
        /// However, unions never have niches and even allow undef,
        /// so there is no `valid_range`.
        value: Primitive,
    },
}
