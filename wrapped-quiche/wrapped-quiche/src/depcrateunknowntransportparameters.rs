// Generated macro for UnknownTransportParameters (struct)
macro_rules! DepcrateUnknownTransportParameters {
() => {
// Module: crate
// Provides: {"UnknownTransportParameters"}
// Dependencies: {}
# [doc = " Track unknown transport parameters, up to a limit."] # [derive (Clone , Debug , PartialEq , Default)] pub struct UnknownTransportParameters { # [doc = " The space remaining for storing unknown transport parameters."] pub capacity : usize , # [doc = " The unknown transport parameters."] pub parameters : Vec < UnknownTransportParameter < Vec < u8 > > > , }
};
}
