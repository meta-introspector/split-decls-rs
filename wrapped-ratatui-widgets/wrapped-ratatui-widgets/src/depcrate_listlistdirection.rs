// Generated macro for ListDirection (enum)
macro_rules! Depcrate_listListDirection {
() => {
// Module: crate::list
// Provides: {"ListDirection"}
// Dependencies: {}
# [doc = " Defines the direction in which the list will be rendered."] # [doc = ""] # [doc = " If there are too few items to fill the screen, the list will stick to the starting edge."] # [doc = ""] # [doc = " See [`List::direction`]."] # [derive (Debug , Default , Display , EnumString , Clone , Copy , Eq , PartialEq , Hash)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub enum ListDirection { # [doc = " The first value is on the top, going to the bottom"] # [default] TopToBottom , # [doc = " The first value is on the bottom, going to the top."] BottomToTop , }
};
}
