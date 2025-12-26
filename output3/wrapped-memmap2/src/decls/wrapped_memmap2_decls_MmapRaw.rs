use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A handle to a raw memory mapped buffer.
///
/// This struct never hands out references to its interior, only raw pointers.
/// This can be helpful when creating shared memory maps between untrusted processes.
///
/// For the safety concerns that arise when converting these raw pointers to references,
/// see the [`Mmap`] safety documentation.
pub struct MmapRaw {
    inner: MmapInner,
}
