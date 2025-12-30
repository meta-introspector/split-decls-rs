// Generated macro for ScrollDirection (enum)
macro_rules! Depcrate_scrollbarScrollDirection {
() => {
// Module: crate::scrollbar
// Provides: {"ScrollDirection"}
// Dependencies: {}
# [doc = " An enum representing a scrolling direction."] # [doc = ""] # [doc = " This is used with [`ScrollbarState::scroll`]."] # [doc = ""] # [doc = " It is useful for example when you want to store in which direction to scroll."] # [derive (Debug , Default , Display , EnumString , Clone , Copy , Eq , PartialEq , Hash)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub enum ScrollDirection { # [doc = " Forward scroll direction, usually corresponds to scrolling downwards or rightwards."] # [default] Forward , # [doc = " Backward scroll direction, usually corresponds to scrolling upwards or leftwards."] Backward , }
};
}
