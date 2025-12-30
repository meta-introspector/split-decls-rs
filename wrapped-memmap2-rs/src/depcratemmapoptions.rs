// Generated macro for MmapOptions (struct)
macro_rules! DepcrateMmapOptions {
() => {
// Module: crate
// Provides: {"MmapOptions"}
// Dependencies: {}
# [doc = " A memory map builder, providing advanced options and flags for specifying memory map behavior."] # [doc = ""] # [doc = " `MmapOptions` can be used to create an anonymous memory map using [`map_anon()`], or a"] # [doc = " file-backed memory map using one of [`map()`], [`map_mut()`], [`map_exec()`],"] # [doc = " [`map_copy()`], or [`map_copy_read_only()`]."] # [doc = ""] # [doc = " ## Safety"] # [doc = ""] # [doc = " All file-backed memory map constructors are marked `unsafe` because of the potential for"] # [doc = " *Undefined Behavior* (UB) using the map if the underlying file is subsequently modified, in or"] # [doc = " out of process. Applications must consider the risk and take appropriate precautions when"] # [doc = " using file-backed maps. Solutions such as file permissions, locks or process-private (e.g."] # [doc = " unlinked) files exist but are platform specific and limited."] # [doc = ""] # [doc = " [`map_anon()`]: MmapOptions::map_anon()"] # [doc = " [`map()`]: MmapOptions::map()"] # [doc = " [`map_mut()`]: MmapOptions::map_mut()"] # [doc = " [`map_exec()`]: MmapOptions::map_exec()"] # [doc = " [`map_copy()`]: MmapOptions::map_copy()"] # [doc = " [`map_copy_read_only()`]: MmapOptions::map_copy_read_only()"] # [derive (Clone , Debug , Default)] pub struct MmapOptions { offset : u64 , len : Option < usize > , huge : Option < u8 > , stack : bool , populate : bool , no_reserve_swap : bool , }
};
}
