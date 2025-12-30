// Generated macro for Decoration (enum)
macro_rules! Depcrate_internal_description_rendererDecoration {
() => {
// Module: crate::internal::description_renderer
// Provides: {"Decoration"}
// Dependencies: {}
# [doc = " The decoration which appears on [`Block`] of a [`List`] when rendered."] # [derive (Debug , Default)] enum Decoration { # [doc = " No decoration on each [`Block`]. The default."] # [default] None , # [doc = " Each [`Block`] is preceded by a bullet (`* `)."] Bullet , # [doc = " Each [`Block`] is preceded by its index in the [`List`] (`0. `, `1. `,"] # [doc = " ...)."] Enumerate , }
};
}
