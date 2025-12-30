// Generated macro for macro_156 (macro)
macro_rules! Depcrate_generatedmacro_156 {
() => {
// Module: crate::generated
// Provides: {"macro_156"}
// Dependencies: {}
extern_class ! (# [doc = " Reader session for processing Value Added Service (VAS) tags.  This session requires the \"com.apple.developer.nfc.readersession.formats\""] # [doc = " entitlement in your process.  In addition your application's Info.plist must contain a non-empty usage description string."] # [doc = ""] # [doc = " ```text"] # [doc = "  NFCReaderErrorSecurityViolation @link/ will be returned from @link [NFCVASReaderSessionDelegate readerSession:didInvalidateWithError:] @link/"] # [doc = "              if the required entitlement is missing when session is started."] # [doc = ""] # [doc = "  NOTE:"] # [doc = "  Only one NFCReaderSession can be active at any time in the system. Subsequent opened sessions will get queued up and processed by the system in FIFO order."] # [doc = "  "] # [doc = ""] # [doc = " ```"] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/corenfc/nfcvasreadersession?language=objc)"] # [unsafe (super (NFCReaderSession , NSObject))] # [derive (Debug , PartialEq , Eq , Hash)] pub struct NFCVASReaderSession ;) ;
};
}
