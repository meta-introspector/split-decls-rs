// Generated macro for default_frame_filter (function)
macro_rules! Depcrate_configdefault_frame_filter {
() => {
// Module: crate::config
// Provides: {"default_frame_filter"}
// Dependencies: {}
fn default_frame_filter (frames : & mut Vec < & Frame >) { let top_cutoff = frames . iter () . rposition (| x | x . is_post_panic_code ()) . map (| x | x + 2) . unwrap_or (0) ; let bottom_cutoff = frames . iter () . position (| x | x . is_runtime_init_code ()) . unwrap_or (frames . len ()) ; let rng = top_cutoff ..= bottom_cutoff ; frames . retain (| x | rng . contains (& x . n)) }
};
}
