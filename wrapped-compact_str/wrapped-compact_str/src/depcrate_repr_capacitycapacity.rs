// Generated macro for Capacity (struct)
macro_rules! Depcrate_repr_capacityCapacity {
() => {
// Module: crate::repr::capacity
// Provides: {"Capacity"}
// Dependencies: {}
# [doc = " An integer type that uses `core::mem::size_of::<usize>() - 1` bytes to store the capacity of"] # [doc = " a heap buffer."] # [doc = ""] # [doc = " Assuming a 64-bit arch, a [`super::BoxString`] uses 8 bytes for a pointer, 8 bytes for a"] # [doc = " length, and then needs 1 byte for a discriminant. We need to store the capacity somewhere, and"] # [doc = " we could store it on the heap, but we also have 7 unused bytes. [`Capacity`] handles storing a"] # [doc = " value in these 7 bytes, returning an error if it's not possible, at which point we'll store the"] # [doc = " capacity on the heap."] # [doc = ""] # [doc = " # Max Values"] # [doc = " * __64-bit:__ `(2 ^ (7 * 8)) - 2 = 72_057_594_037_927_934 ~= 64 petabytes`"] # [doc = " * __32-bit:__ `(2 ^ (3 * 8)) - 2 = 16_777_214             ~= 16 megabytes`"] # [doc = ""] # [doc = " Practically speaking, on a 64-bit architecture we'll never need to store the capacity on the"] # [doc = " heap, because with it's impossible to create a string that is 64 petabytes or larger. But for"] # [doc = " 32-bit architectures we need to be able to store a capacity larger than 16 megabytes, since a"] # [doc = " string larger than 16 megabytes probably isn't that uncommon."] # [derive (Copy , Clone , PartialEq , Eq)] # [repr (transparent)] pub (crate) struct Capacity (usize) ;
};
}
