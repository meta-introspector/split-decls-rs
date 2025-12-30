// Generated macro for Encode (trait)
macro_rules! DepcrateEncode {
() => {
// Module: crate
// Provides: {"Encode"}
// Dependencies: {}
# [cfg (feature = "bincode")] pub trait Encode { fn encode < W : Write > (& self , writer : W) -> Result < () > ; }
};
}
