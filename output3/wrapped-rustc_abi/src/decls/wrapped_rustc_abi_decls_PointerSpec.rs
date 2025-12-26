use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// How pointers are represented in a given address space
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PointerSpec {
    /// The size of the bitwise representation of the pointer.
    pointer_size: Size,
    /// The alignment of pointers for this address space
    pointer_align: Align,
    /// The size of the value a pointer can be offset by in this address space.
    pointer_offset: Size,
    /// Pointers into this address space contain extra metadata
    /// FIXME(workingjubilee): Consider adequately reflecting this in the compiler?
    _is_fat: bool,
}
