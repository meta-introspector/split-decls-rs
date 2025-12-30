// Generated macro for buffer_size (function)
macro_rules! Depcratebuffer_size {
() => {
// Module: crate
// Provides: {"buffer_size"}
// Dependencies: {}
pub fn buffer_size () -> Result < (u16 , u16) > { crossterm :: terminal :: size () }
};
}
