// Generated macro for ErrorKind (enum)
macro_rules! DepcrateErrorKind {
() => {
// Module: crate
// Provides: {"ErrorKind"}
// Dependencies: {}
# [doc = " CAN error kind"] # [doc = ""] # [doc = " This represents a common set of CAN operation errors. HAL implementations are"] # [doc = " free to define more specific or additional error types. However, by providing"] # [doc = " a mapping to these common CAN errors, generic code can still react to them."] # [derive (Debug , Copy , Clone , Eq , PartialEq , Ord , PartialOrd , Hash)] # [cfg_attr (feature = "defmt" , derive (defmt :: Format))] # [non_exhaustive] pub enum ErrorKind { # [doc = " The peripheral receive buffer was overrun."] Overrun , # [doc = " A bit error is detected at that bit time when the bit value that is"] # [doc = " monitored differs from the bit value sent."] Bit , # [doc = " A stuff error is detected at the bit time of the sixth consecutive"] # [doc = " equal bit level in a frame field that shall be coded by the method"] # [doc = " of bit stuffing."] Stuff , # [doc = " Calculated CRC sequence does not equal the received one."] Crc , # [doc = " A form error shall be detected when a fixed-form bit field contains"] # [doc = " one or more illegal bits."] Form , # [doc = " An ACK  error shall be detected by a transmitter whenever it does not"] # [doc = " monitor a dominant bit during the ACK slot."] Acknowledge , # [doc = " A different error occurred. The original error may contain more information."] Other , }
};
}
