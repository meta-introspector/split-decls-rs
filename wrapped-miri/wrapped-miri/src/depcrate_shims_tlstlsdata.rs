// Generated macro for TlsData (struct)
macro_rules! Depcrate_shims_tlsTlsData {
() => {
// Module: crate::shims::tls
// Provides: {"TlsData"}
// Dependencies: {}
# [derive (Debug)] pub struct TlsData < 'tcx > { # [doc = " The Key to use for the next thread-local allocation."] next_key : TlsKey , # [doc = " pthreads-style thread-local storage."] keys : BTreeMap < TlsKey , TlsEntry < 'tcx > > , # [doc = " On macOS, each thread holds a list of destructor functions with their"] # [doc = " respective data arguments."] macos_thread_dtors : BTreeMap < ThreadId , Vec < (ty :: Instance < 'tcx > , Scalar) > > , }
};
}
