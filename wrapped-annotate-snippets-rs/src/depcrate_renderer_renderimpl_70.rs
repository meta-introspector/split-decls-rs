// Generated macro for impl_70 (impl)
macro_rules! Depcrate_renderer_renderimpl_70 {
() => {
// Module: crate::renderer::render
// Provides: {"impl_70"}
// Dependencies: {}
impl ElementStyle { pub (crate) fn color_spec (& self , level : & Level < '_ > , stylesheet : & Stylesheet) -> Style { match self { ElementStyle :: Addition => stylesheet . addition , ElementStyle :: Removal => stylesheet . removal , ElementStyle :: LineAndColumn => stylesheet . none , ElementStyle :: LineNumber => stylesheet . line_num , ElementStyle :: Quotation => stylesheet . none , ElementStyle :: MainHeaderMsg => stylesheet . emphasis , ElementStyle :: UnderlinePrimary | ElementStyle :: LabelPrimary => level . style (stylesheet) , ElementStyle :: UnderlineSecondary | ElementStyle :: LabelSecondary => stylesheet . context , ElementStyle :: HeaderMsg | ElementStyle :: NoStyle => stylesheet . none , ElementStyle :: Level (lvl) => lvl . style (stylesheet) , } } }
};
}
