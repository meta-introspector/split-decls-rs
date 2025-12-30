// Generated macro for impl_205 (impl)
macro_rules! Depcrate_flatimpl_205 {
() => {
// Module: crate::flat
// Provides: {"impl_205"}
// Dependencies: {}
impl < Buffer > IndexMut < (u8 , u32 , u32) > for FlatSamples < Buffer > where Buffer : IndexMut < usize > , { # [doc = " Return a mutable reference to a single sample at specified coordinates."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " When the coordinates are out of bounds or the index calculation fails."] fn index_mut (& mut self , (c , x , y) : (u8 , u32 , u32)) -> & mut Self :: Output { let bounds = self . bounds () ; let strides = self . strides_cwh () ; let index = self . index (c , x , y) . unwrap_or_else (| | panic_cwh_out_of_bounds ((c , x , y) , bounds , strides)) ; & mut self . samples [index] } }
};
}
