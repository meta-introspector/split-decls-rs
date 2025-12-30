// Generated macro for convert_size (function)
macro_rules! Depcrate_plot_plotters_backendconvert_size {
() => {
// Module: crate::plot::plotters_backend
// Provides: {"convert_size"}
// Dependencies: {}
fn convert_size (size : Option < (usize , usize) >) -> Option < (u32 , u32) > { if let Some ((w , h)) = size { return Some ((w as u32 , h as u32)) ; } None }
};
}
