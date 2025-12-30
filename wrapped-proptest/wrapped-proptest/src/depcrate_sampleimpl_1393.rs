// Generated macro for impl_1393 (impl)
macro_rules! Depcrate_sampleimpl_1393 {
() => {
// Module: crate::sample
// Provides: {"impl_1393"}
// Dependencies: {}
impl Index { # [doc = " Return the real index that would be used to index a collection of size `size`."] # [doc = ""] # [doc = " ## Panics"] # [doc = ""] # [doc = " Panics if `size == 0`."] pub fn index (& self , size : usize) -> usize { assert ! (size > 0 , "Attempt to use `Index` with 0-size collection") ; ((size as u128) * (self . 0 as u128) >> (mem :: size_of :: < usize > () * 8)) as usize } # [doc = " Return a reference to the element in `slice` that this `Index` refers to."] # [doc = ""] # [doc = " A shortcut for `&slice[index.index(slice.len())]`."] pub fn get < 'a , T > (& self , slice : & 'a [T]) -> & 'a T { & slice [self . index (slice . len ())] } # [doc = " Return a mutable reference to the element in `slice` that this `Index`"] # [doc = " refers to."] # [doc = ""] # [doc = " A shortcut for `&mut slice[index.index(slice.len())]`."] pub fn get_mut < 'a , T > (& self , slice : & 'a mut [T]) -> & 'a mut T { let ix = self . index (slice . len ()) ; & mut slice [ix] } }
};
}
