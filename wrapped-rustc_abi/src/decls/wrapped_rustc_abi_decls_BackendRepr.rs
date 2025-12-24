use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The way we represent values to the backend
///
/// Previously this was conflated with the "ABI" a type is given, as in the platform-specific ABI.
/// In reality, this implies little about that, but is mostly used to describe the syntactic form
/// emitted for the backend, as most backends handle SSA values and blobs of memory differently.
/// The psABI may need consideration in doing so, but this enum does not constitute a promise for
/// how the value will be lowered to the calling convention, in itself.
///
/// Generally, a codegen backend will prefer to handle smaller values as a scalar or short vector,
/// and larger values will usually prefer to be represented as memory.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "nightly", derive(HashStable_Generic))]
pub enum BackendRepr {
    Scalar(Scalar),
    ScalarPair(Scalar, Scalar),
    SimdVector { element: Scalar, count: u64 },
    Memory {
        /// If true, the size is exact, otherwise it's only a lower bound.
        sized: bool,
    },
}
