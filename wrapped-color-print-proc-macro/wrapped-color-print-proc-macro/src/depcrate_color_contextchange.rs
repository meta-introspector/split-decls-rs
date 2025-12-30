// Generated macro for Change (enum)
macro_rules! Depcrate_color_contextChange {
() => {
// Module: crate::color_context
// Provides: {"Change"}
// Dependencies: {}
# [doc = " A single change to be done inside a tag. Tags with multiple keywords like `<red;bold>` will"] # [doc = " have multiple [`Change`]s."] # [derive (Debug , PartialEq , Clone)] pub enum Change { Foreground (Color) , Background (Color) , Bold , Dim , Underline , Italics , Blink , Strike , Reverse , Conceal , }
};
}
