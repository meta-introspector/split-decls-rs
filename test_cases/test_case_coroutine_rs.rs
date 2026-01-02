// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_abi/src/layout/coroutine.rs
// Error: expected square brackets
// Problematic line: line 28

use rustc_index::{Idx, IndexSlice, IndexVec};
use tracing::{debug, trace};

use crate::{
    BackendRepr, FieldsShape, HasDataLayout, Integer, LayoutData, Primitive, ReprOptions, Scalar,
    StructKind, TagEncoding, Variants, WrappingRange,
};
