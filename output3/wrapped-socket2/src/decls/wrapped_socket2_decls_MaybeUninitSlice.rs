use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A version of [`IoSliceMut`] that allows the buffer to be uninitialised.
///
/// [`IoSliceMut`]: std::io::IoSliceMut
#[repr(transparent)]
pub struct MaybeUninitSlice<'a>(sys::MaybeUninitSlice<'a>);
