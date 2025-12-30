// Generated macro for impl_975 (impl)
macro_rules! Depcrate_runtime_anyprotocolimpl_975 {
() => {
// Module: crate::runtime::anyprotocol
// Provides: {"impl_975"}
// Dependencies: {}
impl PartialEq for AnyProtocol { # [doc = " Check whether the protocols are equal, or conform to each other."] # [inline] # [doc (alias = "protocol_isEqual")] fn eq (& self , other : & Self) -> bool { unsafe { ffi :: protocol_isEqual (self , other) . as_bool () } } }
};
}
