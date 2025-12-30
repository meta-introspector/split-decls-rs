// Generated macro for impl_204 (impl)
macro_rules! Depcrate_flatimpl_204 {
() => {
// Module: crate::flat
// Provides: {"impl_204"}
// Dependencies: {}
impl < Buffer > Index < (u8 , u32 , u32) > for FlatSamples < Buffer > where Buffer : Index < usize > , { type Output = Buffer :: Output ; # [doc = " Return a reference to a single sample at specified coordinates."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " When the coordinates are out of bounds or the index calculation fails."] fn index (& self , (c , x , y) : (u8 , u32 , u32)) -> & Self :: Output { let bounds = self . bounds () ; let strides = self . strides_cwh () ; let index = self . index (c , x , y) . unwrap_or_else (| | panic_cwh_out_of_bounds ((c , x , y) , bounds , strides)) ; & self . samples [index] } }
};
}
