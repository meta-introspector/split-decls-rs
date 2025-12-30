// Generated macro for macro_206 (macro)
macro_rules! Depcrate_streammacro_206 {
() => {
// Module: crate::stream
// Provides: {"macro_206"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`StreamExt::zip()`] method."] # [derive (Clone , Debug)] # [must_use = "streams do nothing unless polled"] pub struct Zip < A : Stream , B > { item_slot : Option < A :: Item >, # [pin] first : A , # [pin] second : B , } }
};
}
