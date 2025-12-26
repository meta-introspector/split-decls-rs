use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A memory map builder, providing advanced options and flags for specifying memory map behavior.
///
/// `MmapOptions` can be used to create an anonymous memory map using [`map_anon()`], or a
/// file-backed memory map using one of [`map()`], [`map_mut()`], [`map_exec()`],
/// [`map_copy()`], or [`map_copy_read_only()`].
///
/// ## Safety
///
/// All file-backed memory map constructors are marked `unsafe` because of the potential for
/// *Undefined Behavior* (UB) using the map if the underlying file is subsequently modified, in or
/// out of process. Applications must consider the risk and take appropriate precautions when
/// using file-backed maps. Solutions such as file permissions, locks or process-private (e.g.
/// unlinked) files exist but are platform specific and limited.
///
/// [`map_anon()`]: MmapOptions::map_anon()
/// [`map()`]: MmapOptions::map()
/// [`map_mut()`]: MmapOptions::map_mut()
/// [`map_exec()`]: MmapOptions::map_exec()
/// [`map_copy()`]: MmapOptions::map_copy()
/// [`map_copy_read_only()`]: MmapOptions::map_copy_read_only()
#[derive(Clone, Debug, Default)]
pub struct MmapOptions {
    offset: u64,
    len: Option<usize>,
    huge: Option<u8>,
    stack: bool,
    populate: bool,
    no_reserve_swap: bool,
}
