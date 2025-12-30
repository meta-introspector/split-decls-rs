// Generated macro for impl_206 (impl)
macro_rules! Depcrate_linear_mapimpl_206 {
() => {
// Module: crate::linear_map
// Provides: {"impl_206"}
// Dependencies: {}
impl < K , V , const N : usize > LinearMap < K , V , N > { # [doc = " Creates an empty `LinearMap`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use heapless::LinearMap;"] # [doc = ""] # [doc = " // allocate the map on the stack"] # [doc = " let mut map: LinearMap<&str, isize, 8> = LinearMap::new();"] # [doc = ""] # [doc = " // allocate the map in a static variable"] # [doc = " static mut MAP: LinearMap<&str, isize, 8> = LinearMap::new();"] # [doc = " ```"] pub const fn new () -> Self { Self { buffer : Vec :: new () } } }
};
}
