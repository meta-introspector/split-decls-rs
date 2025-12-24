use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An arena that can hold objects of only one type.
pub struct TypedArena<T> {
    /// A pointer to the next object to be allocated.
    ptr: Cell<*mut T>,
    /// A pointer to the end of the allocated area. When this pointer is
    /// reached, a new chunk is allocated.
    end: Cell<*mut T>,
    /// A vector of arena chunks.
    chunks: RefCell<Vec<ArenaChunk<T>>>,
    /// Marker indicating that dropping the arena causes its owned
    /// instances of `T` to be dropped.
    _own: PhantomData<T>,
}
