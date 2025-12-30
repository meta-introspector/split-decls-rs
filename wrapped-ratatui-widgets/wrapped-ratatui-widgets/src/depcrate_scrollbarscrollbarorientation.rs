// Generated macro for ScrollbarOrientation (enum)
macro_rules! Depcrate_scrollbarScrollbarOrientation {
() => {
// Module: crate::scrollbar
// Provides: {"ScrollbarOrientation"}
// Dependencies: {}
# [doc = " This is the position of the scrollbar around a given area."] # [doc = ""] # [doc = " ```plain"] # [doc = "           HorizontalTop"] # [doc = "             ┌───────┐"] # [doc = " VerticalLeft│       │VerticalRight"] # [doc = "             └───────┘"] # [doc = "          HorizontalBottom"] # [doc = " ```"] # [derive (Debug , Default , Display , EnumString , Clone , Eq , PartialEq , Hash)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub enum ScrollbarOrientation { # [doc = " Positions the scrollbar on the right, scrolling vertically"] # [default] VerticalRight , # [doc = " Positions the scrollbar on the left, scrolling vertically"] VerticalLeft , # [doc = " Positions the scrollbar on the bottom, scrolling horizontally"] HorizontalBottom , # [doc = " Positions the scrollbar on the top, scrolling horizontally"] HorizontalTop , }
};
}
