// Generated macro for color_of_outcome (function)
macro_rules! Depcrate_printercolor_of_outcome {
() => {
// Module: crate::printer
// Provides: {"color_of_outcome"}
// Dependencies: {}
# [doc = " Returns the `ColorSpec` associated with the given outcome."] fn color_of_outcome (outcome : & Outcome) -> Style { let color = match outcome { Outcome :: Passed => AnsiColor :: Green , Outcome :: Failed { .. } => AnsiColor :: Red , Outcome :: Ignored => AnsiColor :: Yellow , Outcome :: Measured { .. } => AnsiColor :: Cyan , } ; Style :: new () . fg_color (Some (Color :: Ansi (color))) }
};
}
