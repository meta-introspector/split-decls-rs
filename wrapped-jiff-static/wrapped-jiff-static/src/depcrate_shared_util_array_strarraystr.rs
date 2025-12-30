// Generated macro for ArrayStr (struct)
macro_rules! Depcrate_shared_util_array_strArrayStr {
() => {
// Module: crate::shared::util::array_str
// Provides: {"ArrayStr"}
// Dependencies: {}
# [doc = " A simple and not the most-efficient fixed size string on the stack."] # [doc = ""] # [doc = " This supplanted some uses of `Box<str>` for storing tiny strings in an"] # [doc = " effort to reduce our dependence on dynamic memory allocation."] # [doc = ""] # [doc = " Also, since it isn't needed and it lets us save on storage requirements,"] # [doc = " `N` must be less than `256` (so that the length can fit in a `u8`)."] # [derive (Clone , Copy , Eq , Hash , PartialEq , PartialOrd , Ord)] # [doc (hidden)] pub struct ArrayStr < const N : usize > { # [doc = " The UTF-8 bytes that make up the string."] # [doc = ""] # [doc = " This array---the entire array---is always valid UTF-8. And"] # [doc = " the `0..self.len` sub-slice is also always valid UTF-8."] bytes : [u8 ; N] , # [doc = " The number of bytes used by the string in `bytes`."] # [doc = ""] # [doc = " (We could technically save this byte in some cases and use a NUL"] # [doc = " terminator. For example, since we don't permit NUL bytes in POSIX time"] # [doc = " zone abbreviation strings, but this is simpler and only one byte and"] # [doc = " generalizes. And we're not really trying to micro-optimize the storage"] # [doc = " requirements when we use these array strings. Or at least, I don't know"] # [doc = " of a reason to.)"] len : u8 , }
};
}
