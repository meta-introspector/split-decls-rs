// Generated macro for index_alloc (module)
macro_rules! Depcrate_drivers_virtio_virtqueueindex_alloc {
() => {
// Module: crate::drivers::virtio::virtqueue
// Provides: {"index_alloc"}
// Dependencies: {}
mod index_alloc { use alloc :: boxed :: Box ; # [doc = " This type allows allocating indices."] # [doc = ""] # [doc = " The indices can be used as descriptor IDs."] pub struct IndexAlloc { # [doc = " Zero bits are available."] bits : Box < [usize] > , } const USIZE_BITS : usize = usize :: BITS as usize ; impl IndexAlloc { pub fn new (len : usize) -> Self { let usizes = len . div_ceil (USIZE_BITS) ; let extra_bits = len % USIZE_BITS ; let mut bits = vec ! [0 ; usizes] . into_boxed_slice () ; if extra_bits != 0 { * bits . last_mut () . unwrap () = usize :: MAX >> extra_bits ; } Self { bits } } # [inline] pub fn allocate (& mut self) -> Option < usize > { for (word_index , word) in self . bits . iter_mut () . enumerate () { let trailing_ones = word . trailing_ones () ; if trailing_ones < usize :: BITS { let mask = 1 << trailing_ones ; * word |= mask ; let index = word_index * USIZE_BITS + usize :: try_from (trailing_ones) . unwrap () ; return Some (index) ; } } None } # [inline] pub unsafe fn deallocate (& mut self , index : usize) { let word_index = index / USIZE_BITS ; let bit = index % USIZE_BITS ; let mask = 1 << bit ; debug_assert ! (self . bits [word_index] & mask == mask) ; unsafe { * self . bits . get_unchecked_mut (word_index) &= ! mask ; } } } }
};
}
