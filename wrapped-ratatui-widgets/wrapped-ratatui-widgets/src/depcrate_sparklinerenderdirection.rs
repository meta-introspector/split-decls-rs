// Generated macro for RenderDirection (enum)
macro_rules! Depcrate_sparklineRenderDirection {
() => {
// Module: crate::sparkline
// Provides: {"RenderDirection"}
// Dependencies: {}
# [doc = " Defines the direction in which sparkline will be rendered."] # [doc = ""] # [doc = " See [`Sparkline::direction`]."] # [derive (Debug , Default , Display , EnumString , Clone , Copy , Eq , PartialEq , Hash)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub enum RenderDirection { # [doc = " The first value is on the left, going to the right"] # [default] LeftToRight , # [doc = " The first value is on the right, going to the left"] RightToLeft , }
};
}
