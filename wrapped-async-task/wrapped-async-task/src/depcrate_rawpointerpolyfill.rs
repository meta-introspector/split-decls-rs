// Generated macro for PointerPolyfill (trait)
macro_rules! Depcrate_rawPointerPolyfill {
() => {
// Module: crate::raw
// Provides: {"PointerPolyfill"}
// Dependencies: {}
trait PointerPolyfill { # [doc = " Adds an unsigned offset in bytes to a pointer."] # [doc = ""] # [doc = " `count` is in units of bytes."] # [doc = ""] # [doc = " This is purely a convenience for casting to a `u8` pointer and"] # [doc = " using [add][pointer::add] on it. See that method for documentation"] # [doc = " and safety requirements."] # [doc = ""] # [doc = " # Safety"] # [doc = " If any of the following conditions are violated, the result is Undefined Behavior:"] # [doc = ""] # [doc = "  - The offset in bytes, count * size_of::<T>(), computed on mathematical integers"] # [doc = "    (without “wrapping around”), must fit in an isize."] # [doc = "  - If the computed offset is non-zero, then self must be derived from a pointer to"] # [doc = "    some allocation, and the entire memory range between self and the result must be"] # [doc = "    in bounds of that allocation. In particular, this range must not “wrap around”"] # [doc = "    the edge of the address space."] unsafe fn add_byte (self , size : usize) -> Self ; }
};
}
