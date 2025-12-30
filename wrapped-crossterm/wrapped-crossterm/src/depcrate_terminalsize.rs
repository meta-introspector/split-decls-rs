// Generated macro for size (function)
macro_rules! Depcrate_terminalsize {
() => {
// Module: crate::terminal
// Provides: {"size"}
// Dependencies: {}
# [doc = " Returns the terminal size `(columns, rows)`."] # [doc = ""] # [doc = " The top left cell is represented `(1, 1)`."] pub fn size () -> io :: Result < (u16 , u16) > { sys :: size () }
};
}
