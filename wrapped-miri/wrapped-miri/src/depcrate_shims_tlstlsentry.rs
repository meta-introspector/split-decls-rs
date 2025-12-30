// Generated macro for TlsEntry (struct)
macro_rules! Depcrate_shims_tlsTlsEntry {
() => {
// Module: crate::shims::tls
// Provides: {"TlsEntry"}
// Dependencies: {}
# [derive (Clone , Debug)] pub struct TlsEntry < 'tcx > { # [doc = " The data for this key. None is used to represent NULL."] # [doc = " (We normalize this early to avoid having to do a NULL-ptr-test each time we access the data.)"] data : BTreeMap < ThreadId , Scalar > , dtor : Option < ty :: Instance < 'tcx > > , }
};
}
