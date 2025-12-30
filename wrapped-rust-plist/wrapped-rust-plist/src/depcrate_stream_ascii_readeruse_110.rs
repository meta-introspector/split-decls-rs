// Generated macro for use_110 (use)
macro_rules! Depcrate_stream_ascii_readeruse_110 {
() => {
// Module: crate::stream::ascii_reader
// Provides: {"use_110"}
// Dependencies: {}
# [doc = " Ascii property lists are used in legacy settings and only support four"] # [doc = " datatypes: Array, Dictionary, String and Data."] # [doc = " See [Apple"] # [doc = " Documentation](https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/PropertyLists/OldStylePlists/OldStylePLists.html)"] # [doc = " for more info."] # [doc = " However this reader also support Integers as first class datatype."] # [doc = " This reader will accept certain ill-formed ascii plist without complaining."] # [doc = " It does not check the integrity of the plist format."] use crate :: { error :: { Error , ErrorKind } , stream :: { Event , OwnedEvent } , Integer , } ;
};
}
