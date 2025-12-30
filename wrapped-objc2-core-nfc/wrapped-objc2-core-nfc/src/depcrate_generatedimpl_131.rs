// Generated macro for impl_131 (impl)
macro_rules! Depcrate_generatedimpl_131 {
() => {
// Module: crate::generated
// Provides: {"impl_131"}
// Dependencies: {}
impl NFCNDEFMessage { extern_methods ! (# [doc = " Array of NFCNDEFPayload records contained in this message."] # [unsafe (method (records))] # [unsafe (method_family = none)] pub unsafe fn records (& self) -> Retained < NSArray < NFCNDEFPayload >>; # [doc = " Setter for [`records`][Self::records]."] # [doc = ""] # [doc = " This is [copied][objc2_foundation::NSCopying::copy] when set."] # [unsafe (method (setRecords :))] # [unsafe (method_family = none)] pub unsafe fn setRecords (& self , records : & NSArray < NFCNDEFPayload >) ; # [doc = " Length of the resulting NDEF message in bytes as it would be stored on a NFC tag."] # [unsafe (method (length))] # [unsafe (method_family = none)] pub unsafe fn length (& self) -> NSUInteger ; # [unsafe (method (init))] # [unsafe (method_family = init)] pub unsafe fn init (this : Allocated < Self >) -> Retained < Self >; # [doc = " Parameter `records`: NSArray of NFCNDEFPayload object.  An empty array will create an empty NDEF message."] # [unsafe (method (initWithNDEFRecords :))] # [unsafe (method_family = init)] pub unsafe fn initWithNDEFRecords (this : Allocated < Self >, records : & NSArray < NFCNDEFPayload >,) -> Retained < Self >; # [doc = " Parameter `data`: NSData storing raw bytes of a complete NDEF message.  The data content will be validated; all NDEF payloads must"] # [doc = " be valid according to the NFC Forum NDEF RTD specification and it shall only contain a single NDEF message."] # [unsafe (method (ndefMessageWithData :))] # [unsafe (method_family = none)] pub unsafe fn ndefMessageWithData (data : & NSData) -> Option < Retained < Self >>;) ; }
};
}
