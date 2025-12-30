// Generated macro for overlaps (function)
macro_rules! Depcrate_renderer_renderoverlaps {
() => {
// Module: crate::renderer::render
// Provides: {"overlaps"}
// Dependencies: {}
fn overlaps (a1 : & LineAnnotation < '_ > , a2 : & LineAnnotation < '_ > , padding : usize) -> bool { num_overlap (a1 . start . display , a1 . end . display + padding , a2 . start . display , a2 . end . display , false ,) }
};
}
