// Generated macro for CharAccumulator (trait)
macro_rules! DepcrateCharAccumulator {
() => {
// Module: crate
// Provides: {"CharAccumulator"}
// Dependencies: {}
# [doc = " Build a `char` out of bytes"] pub trait CharAccumulator : Default { # [doc = " Build a `char` out of bytes"] # [doc = ""] # [doc = " Return `None` when more data is needed"] fn add (& mut self , byte : u8) -> Option < char > ; }
};
}
