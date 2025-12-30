// Generated macro for Decoder (struct)
macro_rules! Depcrate_decDecoder {
() => {
// Module: crate::dec
// Provides: {"Decoder"}
// Dependencies: {}
# [doc = " A decoder for deserializing CBOR items"] # [doc = ""] # [doc = " This decoder manages the low-level decoding of CBOR items into `Header`"] # [doc = " objects. It also contains utility functions for parsing segmented bytes"] # [doc = " and text inputs."] pub struct Decoder < R > { reader : R , offset : usize , buffer : Option < Title > , }
};
}
