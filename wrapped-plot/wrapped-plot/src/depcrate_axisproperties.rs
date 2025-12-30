// Generated macro for Properties (struct)
macro_rules! Depcrate_axisProperties {
() => {
// Module: crate::axis
// Provides: {"Properties"}
// Dependencies: {}
# [doc = " Properties of the coordinate axes"] # [derive (Clone)] pub struct Properties { grids : map :: grid :: Map < grid :: Properties > , hidden : bool , label : Option < Cow < 'static , str > > , logarithmic : bool , range : Option < (f64 , f64) > , scale_factor : f64 , tics : Option < String > , }
};
}
