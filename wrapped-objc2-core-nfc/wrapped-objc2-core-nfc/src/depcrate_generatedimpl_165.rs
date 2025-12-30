// Generated macro for impl_165 (impl)
macro_rules! Depcrate_generatedimpl_165 {
() => {
// Module: crate::generated
// Provides: {"impl_165"}
// Dependencies: {}
# [doc = " Methods declared on superclass `NFCTagReaderSession`."] impl NFCPaymentTagReaderSession { extern_methods ! (# [unsafe (method (init))] # [unsafe (method_family = init)] pub unsafe fn init (this : Allocated < Self >) -> Retained < Self >; # [cfg (feature = "dispatch2")] # [doc = " Parameter `pollingOption`: Configures the RF polling of the reader session; multiple options can be OR'ed together.  This option affects the possible NFC tag type discover."] # [doc = ""] # [doc = " Parameter `delegate`: The session will hold a weak ARC reference to this"] # [doc = ""] # [doc = " ```text"] # [doc = "  NFCTagReaderSessionDelegate @link/ object."] # [doc = "  @param queue         A dispatch queue where NFCTagReaderSessionDelegate delegate callbacks will be dispatched to.  A <i>nil</i> value will"] # [doc = "                       cause the creation of a serial dispatch queue internally for the session.  The session object will retain the provided dispatch queue."] # [doc = ""] # [doc = "  @return              A new NFCTagReaderSession instance."] # [doc = "  "] # [doc = ""] # [doc = " ```"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `queue` possibly has additional threading requirements."] # [unsafe (method (initWithPollingOption : delegate : queue :))] # [unsafe (method_family = init)] pub unsafe fn initWithPollingOption_delegate_queue (this : Allocated < Self >, polling_option : NFCPollingOption , delegate : & ProtocolObject < dyn NFCTagReaderSessionDelegate >, queue : Option <& DispatchQueue >,) -> Retained < Self >;) ; }
};
}
