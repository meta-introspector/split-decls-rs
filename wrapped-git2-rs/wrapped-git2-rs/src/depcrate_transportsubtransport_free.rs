// Generated macro for subtransport_free (function)
macro_rules! Depcrate_transportsubtransport_free {
() => {
// Module: crate::transport
// Provides: {"subtransport_free"}
// Dependencies: {}
extern "C" fn subtransport_free (transport : * mut raw :: git_smart_subtransport) { let _ = panic :: wrap (| | unsafe { mem :: transmute :: < _ , Box < RawSmartSubtransport > > (transport) ; }) ; }
};
}
