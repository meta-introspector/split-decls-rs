// Generated macro for UnknownTransportParameter (struct)
macro_rules! DepcrateUnknownTransportParameter {
() => {
// Module: crate
// Provides: {"UnknownTransportParameter"}
// Dependencies: {}
# [doc = " QUIC Unknown Transport Parameter."] # [doc = ""] # [doc = " A QUIC transport parameter that is not specifically recognized"] # [doc = " by this implementation."] # [derive (Clone , Debug , PartialEq)] pub struct UnknownTransportParameter < T > { # [doc = " The ID of the unknown transport parameter."] pub id : u64 , # [doc = " Original data representing the value of the unknown transport parameter."] pub value : T , }
};
}
