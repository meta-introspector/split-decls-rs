// Generated macro for impl_1302 (impl)
macro_rules! Depcrate_readimpl_1302 {
() => {
// Module: crate::read
// Provides: {"impl_1302"}
// Dependencies: {}
impl < 'data > ObjectMap < 'data > { # [doc = " Get the entry containing the given address."] pub fn get (& self , address : u64) -> Option < & ObjectMapEntry < 'data > > { self . symbols . get (address) . filter (| entry | entry . size == 0 || address . wrapping_sub (entry . address) < entry . size) } # [doc = " Get all symbols in the map."] # [inline] pub fn symbols (& self) -> & [ObjectMapEntry < 'data >] { self . symbols . symbols () } # [doc = " Get all objects in the map."] # [inline] pub fn objects (& self) -> & [ObjectMapFile < 'data >] { & self . objects } }
};
}
