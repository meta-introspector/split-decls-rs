// Generated macro for Error (enum)
macro_rules! Depcrate_errorError {
() => {
// Module: crate::error
// Provides: {"Error"}
// Dependencies: {}
# [doc = " This is the error type used by Postcard"] # [derive (Clone , Debug , Eq , PartialEq , Serialize , Deserialize)] # [cfg_attr (feature = "use-defmt" , derive (defmt :: Format))] # [non_exhaustive] pub enum Error { # [doc = " This is a feature that postcard will never implement"] WontImplement , # [doc = " This is a feature that postcard intends to support, but does not yet"] NotYetImplemented , # [doc = " The serialize buffer is full"] SerializeBufferFull , # [doc = " The length of a sequence must be known"] SerializeSeqLengthUnknown , # [doc = " Hit the end of buffer, expected more data"] DeserializeUnexpectedEnd , # [doc = " Found a varint that didn't terminate. Is the usize too big for this platform?"] DeserializeBadVarint , # [doc = " Found a bool that wasn't 0 or 1"] DeserializeBadBool , # [doc = " Found an invalid unicode char"] DeserializeBadChar , # [doc = " Tried to parse invalid utf-8"] DeserializeBadUtf8 , # [doc = " Found an Option discriminant that wasn't 0 or 1"] DeserializeBadOption , # [doc = " Found an enum discriminant that was > `u32::MAX`"] DeserializeBadEnum , # [doc = " The original data was not well encoded"] DeserializeBadEncoding , # [doc = " Bad CRC while deserializing"] DeserializeBadCrc , # [doc = " Serde Serialization Error"] SerdeSerCustom , # [doc = " Serde Deserialization Error"] SerdeDeCustom , # [doc = " Error while processing `collect_str` during serialization"] CollectStrError , }
};
}
