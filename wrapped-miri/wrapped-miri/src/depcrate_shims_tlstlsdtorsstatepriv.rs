// Generated macro for TlsDtorsStatePriv (enum)
macro_rules! Depcrate_shims_tlsTlsDtorsStatePriv {
() => {
// Module: crate::shims::tls
// Provides: {"TlsDtorsStatePriv"}
// Dependencies: {}
# [derive (Debug , Default)] enum TlsDtorsStatePriv < 'tcx > { # [default] Init , MacOsDtors , PthreadDtors (RunningDtorState) , # [doc = " For Windows Dtors, we store the list of functions that we still have to call."] # [doc = " These are functions from the magic `.CRT$XLB` linker section."] WindowsDtors (Vec < ImmTy < 'tcx > >) , Done , }
};
}
