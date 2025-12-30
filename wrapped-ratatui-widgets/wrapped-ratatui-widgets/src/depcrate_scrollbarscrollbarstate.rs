// Generated macro for ScrollbarState (struct)
macro_rules! Depcrate_scrollbarScrollbarState {
() => {
// Module: crate::scrollbar
// Provides: {"ScrollbarState"}
// Dependencies: {}
# [doc = " A struct representing the state of a Scrollbar widget."] # [doc = ""] # [doc = " # Important"] # [doc = ""] # [doc = " It's essential to set the `content_length` field when using this struct. This field"] # [doc = " represents the total length of the scrollable content. The default value is zero"] # [doc = " which will result in the Scrollbar not rendering."] # [doc = ""] # [doc = " For example, in the following list, assume there are 4 bullet points:"] # [doc = ""] # [doc = " - the `content_length` is 4"] # [doc = " - the `position` is 0"] # [doc = " - the `viewport_content_length` is 2"] # [doc = ""] # [doc = " ```text"] # [doc = " ┌───────────────┐"] # [doc = " │1. this is a   █"] # [doc = " │   single item █"] # [doc = " │2. this is a   ║"] # [doc = " │   second item ║"] # [doc = " └───────────────┘"] # [doc = " ```"] # [doc = ""] # [doc = " If you don't have multi-line content, you can leave the `viewport_content_length` set to the"] # [doc = " default and it'll use the track size as a `viewport_content_length`."] # [derive (Debug , Default , Clone , Copy , Eq , PartialEq , Hash)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct ScrollbarState { # [doc = " The total length of the scrollable content."] content_length : usize , # [doc = " The current position within the scrollable content."] position : usize , # [doc = " The length of content in current viewport."] # [doc = ""] # [doc = " FIXME: this should be `Option<usize>`, but it will break serialization to change it."] viewport_content_length : usize , }
};
}
