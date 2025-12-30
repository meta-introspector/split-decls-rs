// Generated macro for Cobs (struct)
macro_rules! Depcrate_ser_flavorsCobs {
() => {
// Module: crate::ser::flavors
// Provides: {"Cobs"}
// Dependencies: {}
# [doc = " The `Cobs` flavor implements [Consistent Overhead Byte Stuffing] on"] # [doc = " the serialized data. The output of this flavor includes the termination/sentinel"] # [doc = " byte of `0x00`."] # [doc = ""] # [doc = " This protocol is useful when sending data over a serial interface without framing such as a UART"] # [doc = ""] # [doc = " [Consistent Overhead Byte Stuffing]: https://en.wikipedia.org/wiki/Consistent_Overhead_Byte_Stuffing"] pub struct Cobs < B > where B : Flavor + IndexMut < usize , Output = u8 > , { flav : B , cobs : EncoderState , }
};
}
