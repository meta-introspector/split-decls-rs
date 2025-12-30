// Generated macro for subtransport_close (function)
macro_rules! Depcrate_transportsubtransport_close {
() => {
// Module: crate::transport
// Provides: {"subtransport_close"}
// Dependencies: {}
extern "C" fn subtransport_close (transport : * mut raw :: git_smart_subtransport) -> c_int { let ret = panic :: wrap (| | unsafe { let transport = & mut * (transport as * mut RawSmartSubtransport) ; transport . obj . close () }) ; match ret { Some (Ok (())) => 0 , Some (Err (e)) => e . raw_code () as c_int , None => - 1 , } }
};
}
