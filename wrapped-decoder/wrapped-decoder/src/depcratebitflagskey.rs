// Generated macro for BitflagsKey (struct)
macro_rules! DepcrateBitflagsKey {
() => {
// Module: crate
// Provides: {"BitflagsKey"}
// Dependencies: {}
# [doc = " Data that uniquely identifies a `defmt::bitflags!` invocation."] # [derive (Debug , PartialEq , Eq , Hash)] struct BitflagsKey { # [doc = " Name of the bitflags struct (this is really redundant with `disambig`)."] ident : String , package : String , disambig : String , crate_name : Option < String > , }
};
}
