// Generated macro for impl_85 (impl)
macro_rules! Depcrate_log_formatimpl_85 {
() => {
// Module: crate::log::format
// Provides: {"impl_85"}
// Dependencies: {}
impl LogSegment { pub (super) const fn new (metadata : LogMetadata) -> Self { Self { metadata , format : LogFormat { color : None , style : None , width : None , alignment : None , padding : None , } , } } # [cfg (test)] pub (crate) const fn with_color (mut self , color : LogColor) -> Self { self . format . color = Some (color) ; self } # [cfg (test)] pub (crate) fn with_style (mut self , style : colored :: Styles) -> Self { let mut styles = self . format . style . unwrap_or_default () ; styles . push (style) ; self . format . style = Some (styles) ; self } # [cfg (test)] pub (crate) const fn with_width (mut self , width : usize) -> Self { self . format . width = Some (width) ; self } # [cfg (test)] pub (crate) const fn with_alignment (mut self , alignment : Alignment) -> Self { self . format . alignment = Some (alignment) ; self } # [cfg (test)] pub (crate) const fn with_padding (mut self , padding : Padding) -> Self { self . format . padding = Some (padding) ; self } }
};
}
