// Generated macro for define_color (macro)
macro_rules! Depcrate_style_colorsdefine_color {
() => {
// Module: crate::style::colors
// Provides: {"define_color"}
// Dependencies: {}
# [doc = " Defines and names a color based on its R, G, B, A values."] # [macro_export] macro_rules ! define_color { ($ name : ident , $ r : expr , $ g : expr , $ b : expr , $ doc : expr) => { doc ! { [$ doc] [concat ! ("(<span style='color: rgb(" , $ r , "," , $ g , "," , $ b , "); background-color: #ddd; padding: 0 0.2em;'>■</span>")] [concat ! ("*rgb = (" , $ r , ", " , $ g , ", " , $ b , ")*)")] @ pub const $ name : RGBColor = RGBColor ($ r , $ g , $ b) ; } } ; ($ name : ident , $ r : expr , $ g : expr , $ b : expr , $ a : expr , $ doc : expr) => { doc ! { [$ doc] [concat ! ("(<span style='color: rgba(" , $ r , "," , $ g , "," , $ b , "," , $ a , "); background-color: #ddd; padding: 0 0.2em;'>■</span>")] [concat ! ("*rgba = (" , $ r , ", " , $ g , ", " , $ b , ", " , $ a , ")*)")] @ pub const $ name : RGBAColor = RGBAColor ($ r , $ g , $ b , $ a) ; } } ; }
};
}
