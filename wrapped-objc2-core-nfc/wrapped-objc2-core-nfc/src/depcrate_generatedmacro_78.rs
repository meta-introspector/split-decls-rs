// Generated macro for macro_78 (macro)
macro_rules! Depcrate_generatedmacro_78 {
() => {
// Module: crate::generated
// Provides: {"macro_78"}
// Dependencies: {}
extern_class ! (# [doc = " Reader session for processing ISO15693 tags."] # [doc = ""] # [doc = " ```text"] # [doc = "  [NFCReaderSessionDelegate readerSession:didDetectTags:] @link/ will return tag objects that"] # [doc = "              are conformed to the NFCISO15693Tag protocol.  This session requires the \"com.apple.developer.nfc.readersession.formats\" entitlement in your process."] # [doc = ""] # [doc = "  NOTE:"] # [doc = "  Only one NFCReaderSession can be active at any time in the system. Subsequent opened sessions will get queued up and processed by the system in FIFO order."] # [doc = "  The NFCISO15693 tag object returned by this session will only respond to the legacy APIs that are introduced in iOS11."] # [doc = "  "] # [doc = ""] # [doc = " ```"] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/corenfc/nfciso15693readersession?language=objc)"] # [unsafe (super (NFCReaderSession , NSObject))] # [derive (Debug , PartialEq , Eq , Hash)] # [deprecated] pub struct NFCISO15693ReaderSession ;) ;
};
}
