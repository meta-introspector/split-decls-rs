// Generated macro for impl_346 (impl)
macro_rules! Depcrate_index_strimpl_346 {
() => {
// Module: crate::index_str
// Provides: {"impl_346"}
// Dependencies: {}
# [doc = " # Range Methods"] # [doc = ""] # [doc = " Unfortunately, `std::ops::Index` *must* return a reference, so we can't"] # [doc = " implement `Index<Range<usize>>` to return a new `IndexStr` the way we would"] # [doc = " like to. Instead, we abandon fancy indexing operators and have these plain"] # [doc = " old methods."] # [doc = ""] # [doc = " All of these methods panic on an out-of-bounds index."] # [allow (dead_code)] impl < 'a > IndexStr < 'a > { # [doc = " Take the given `start..end` range of the underlying string and return a"] # [doc = " new `IndexStr`."] # [inline] pub fn range (& self , idx : Range < usize >) -> IndexStr < 'a > { IndexStr { idx : self . idx + idx . start , string : & self . string [idx] , } } # [doc = " Take the given `start..` range of the underlying string and return a new"] # [doc = " `IndexStr`."] # [inline] pub fn range_from (& self , idx : RangeFrom < usize >) -> IndexStr < 'a > { IndexStr { idx : self . idx + idx . start , string : & self . string [idx] , } } # [doc = " Take the given `..end` range of the underlying string and return a new"] # [doc = " `IndexStr`."] # [inline] pub fn range_to (& self , idx : RangeTo < usize >) -> IndexStr < 'a > { IndexStr { idx : self . idx , string : & self . string [idx] , } } }
};
}
