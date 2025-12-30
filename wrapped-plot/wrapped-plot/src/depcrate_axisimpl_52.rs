// Generated macro for impl_52 (impl)
macro_rules! Depcrate_axisimpl_52 {
() => {
// Module: crate::axis
// Provides: {"impl_52"}
// Dependencies: {}
impl Configure < Grid > for Properties { type Properties = grid :: Properties ; # [doc = " Configures the gridlines"] fn configure < F > (& mut self , grid : Grid , configure : F) -> & mut Properties where F : FnOnce (& mut grid :: Properties) -> & mut grid :: Properties , { if self . grids . contains_key (grid) { configure (self . grids . get_mut (grid) . unwrap ()) ; } else { let mut properties = Default :: default () ; configure (& mut properties) ; self . grids . insert (grid , properties) ; } self } }
};
}
