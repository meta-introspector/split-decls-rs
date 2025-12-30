// Generated macro for newline_count (function)
macro_rules! Depcrate_renderer_rendernewline_count {
() => {
// Module: crate::renderer::render
// Provides: {"newline_count"}
// Dependencies: {}
fn newline_count (body : & str) -> usize { # [cfg (feature = "simd")] { memchr :: memchr_iter (b'\n' , body . as_bytes ()) . count () } # [cfg (not (feature = "simd"))] { body . lines () . count () . saturating_sub (1) } }
};
}
