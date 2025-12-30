// Generated macro for Utf8String (struct)
macro_rules! DepcrateUtf8String {
() => {
// Module: crate
// Provides: {"Utf8String"}
// Dependencies: {}
# [doc = " Represents an UTF-8 MessagePack string type."] # [doc = ""] # [doc = " According to the MessagePack spec, string objects may contain invalid byte sequence and the"] # [doc = " behavior of a deserializer depends on the actual implementation when it received invalid byte"] # [doc = " sequence."] # [doc = " Deserializers should provide functionality to get the original byte array so that applications"] # [doc = " can decide how to handle the object."] # [doc = ""] # [doc = " Summarizing, it's prohibited to instantiate a string type with invalid UTF-8 sequences, however"] # [doc = " it is possible to obtain an underlying bytes that were attempted to convert to a `String`. This"] # [doc = " may happen when trying to unpack strings that were decoded using older MessagePack spec with"] # [doc = " raw types instead of string/binary."] # [derive (Clone , Debug , PartialEq)] pub struct Utf8String { s : Result < String , (Vec < u8 > , Utf8Error) > , }
};
}
