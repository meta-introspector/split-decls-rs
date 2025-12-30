// Generated macro for Renderable (trait)
macro_rules! Depcrate_renderRenderable {
() => {
// Module: crate::render
// Provides: {"Renderable"}
// Dependencies: {}
# [doc = " Render trait"] pub trait Renderable { # [doc = " render into `RenderContext`'s `writer`"] fn render < 'reg : 'rc , 'rc > (& 'rc self , registry : & 'reg Registry < 'reg > , context : & 'rc Context , rc : & mut RenderContext < 'reg , 'rc > , out : & mut dyn Output ,) -> Result < () , RenderError > ; # [doc = " render into string"] fn renders < 'reg : 'rc , 'rc > (& 'rc self , registry : & 'reg Registry < 'reg > , ctx : & 'rc Context , rc : & mut RenderContext < 'reg , 'rc > ,) -> Result < String , RenderError > { let mut so = StringOutput :: new () ; self . render (registry , ctx , rc , & mut so) ? ; so . into_string () . map_err (| e | RenderErrorReason :: from (e) . into ()) } }
};
}
