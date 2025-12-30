// Generated macro for HexDisplay (trait)
macro_rules! Depcrate_traitsHexDisplay {
() => {
// Module: crate::traits
// Provides: {"HexDisplay"}
// Dependencies: {}
# [cfg (feature = "std")] # [cfg_attr (feature = "docsrs" , doc (cfg (feature = "std")))] # [doc = " Helper trait to show a byte slice as a hex dump"] pub trait HexDisplay { # [doc = " Converts the value of `self` to a hex dump, returning the owned"] # [doc = " `String`."] fn to_hex (& self , chunk_size : usize) -> String ; # [doc = " Converts the value of `self` to a hex dump beginning at `from` address, returning the owned"] # [doc = " `String`."] fn to_hex_from (& self , chunk_size : usize , from : usize) -> String ; }
};
}
