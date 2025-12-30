// Generated macro for impl_35 (impl)
macro_rules! Depcrate_displayimpl_35 {
() => {
// Module: crate::display
// Provides: {"impl_35"}
// Dependencies: {}
impl Display < & 'static str > for LineType { fn display (& self) -> & 'static str { match * self { LineType :: Dash => "2" , LineType :: Dot => "3" , LineType :: DotDash => "4" , LineType :: DotDotDash => "5" , LineType :: SmallDot => "0" , LineType :: Solid => "1" , } } }
};
}
