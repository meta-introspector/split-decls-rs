// Generated macro for impl_542 (impl)
macro_rules! Depcrate_output_color_scaleimpl_542 {
() => {
// Module: crate::output::color_scale
// Provides: {"impl_542"}
// Dependencies: {}
impl Extremes { fn update (maybe_value : Option < f32 > , maybe_range : & mut Option < Extremes >) { match (maybe_value , maybe_range) { (Some (value) , Some (range)) => { if value > range . max { range . max = value ; } else if value < range . min { range . min = value ; } ; } (Some (value) , rel) => { let _ = rel . insert ({ Extremes { max : value , min : value , } }) ; } _ => () , } ; } }
};
}
