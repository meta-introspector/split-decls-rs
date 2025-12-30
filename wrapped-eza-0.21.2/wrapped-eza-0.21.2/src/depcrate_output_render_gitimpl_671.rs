// Generated macro for impl_671 (impl)
macro_rules! Depcrate_output_render_gitimpl_671 {
() => {
// Module: crate::output::render::git
// Provides: {"impl_671"}
// Dependencies: {}
impl f :: Git { pub fn render (self , colours : & dyn Colours) -> TextCell { TextCell { width : DisplayWidth :: from (2) , contents : vec ! [self . staged . render (colours) , self . unstaged . render (colours)] . into () , } } }
};
}
