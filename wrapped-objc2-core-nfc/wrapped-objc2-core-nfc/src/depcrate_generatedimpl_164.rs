// Generated macro for impl_164 (impl)
macro_rules! Depcrate_generatedimpl_164 {
() => {
// Module: crate::generated
// Provides: {"impl_164"}
// Dependencies: {}
impl NFCPaymentTagReaderSession { extern_methods ! (# [cfg (feature = "dispatch2")] # [doc = " Parameter `delegate`: The session will hold a weak ARC reference to this"] # [doc = ""] # [doc = " ```text"] # [doc = "  NFCTagReaderSessionDelegate @link/ object."] # [doc = "  @param queue         A dispatch queue where NFCTagReaderSessionDelegate delegate callbacks will be dispatched to.  A <i>nil</i> value will"] # [doc = "                       cause the creation of a serial dispatch queue internally for the session.  The session object will retain the provided dispatch queue."] # [doc = ""] # [doc = "  @return              A new NFCPaymentTagReaderSession instance."] # [doc = ""] # [doc = "  NOTE:"] # [doc = "  The super class `-initWithPollingOption:delegate:queue:` initializer would only accept NFCPollingOption.NFCPollingISO14443; all other options will be ignored."] # [doc = "  "] # [doc = ""] # [doc = " ```"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `queue` possibly has additional threading requirements."] # [unsafe (method (initWithDelegate : queue :))] # [unsafe (method_family = init)] pub unsafe fn initWithDelegate_queue (this : Allocated < Self >, delegate : & ProtocolObject < dyn NFCTagReaderSessionDelegate >, queue : Option <& DispatchQueue >,) -> Retained < Self >;) ; }
};
}
