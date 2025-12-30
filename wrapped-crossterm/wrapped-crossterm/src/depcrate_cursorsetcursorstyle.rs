// Generated macro for SetCursorStyle (enum)
macro_rules! Depcrate_cursorSetCursorStyle {
() => {
// Module: crate::cursor
// Provides: {"SetCursorStyle"}
// Dependencies: {}
# [doc = " A command that sets the style of the cursor."] # [doc = " It uses two types of escape codes, one to control blinking, and the other the shape."] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " - Commands must be executed/queued for execution otherwise they do nothing."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub enum SetCursorStyle { # [doc = " Default cursor shape configured by the user."] DefaultUserShape , # [doc = " A blinking block cursor shape (■)."] BlinkingBlock , # [doc = " A non blinking block cursor shape (inverse of `BlinkingBlock`)."] SteadyBlock , # [doc = " A blinking underscore cursor shape(_)."] BlinkingUnderScore , # [doc = " A non blinking underscore cursor shape (inverse of `BlinkingUnderScore`)."] SteadyUnderScore , # [doc = " A blinking cursor bar shape (|)"] BlinkingBar , # [doc = " A steady cursor bar shape (inverse of `BlinkingBar`)."] SteadyBar , }
};
}
