// Generated macro for LruSlab (struct)
macro_rules! DepcrateLruSlab {
() => {
// Module: crate
// Provides: {"LruSlab"}
// Dependencies: {}
# [doc = " A random-access table that maintains an LRU list in constant time"] # [derive (Clone)] pub struct LruSlab < T > { slots : Box < [Slot < T >] > , # [doc = " Most recently used"] head : u32 , # [doc = " Least recently used"] tail : u32 , # [doc = " First unused"] free : u32 , # [doc = " Number of occupied slots"] len : u32 , }
};
}
