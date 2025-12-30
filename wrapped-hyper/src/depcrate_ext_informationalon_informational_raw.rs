// Generated macro for on_informational_raw (function)
macro_rules! Depcrate_ext_informationalon_informational_raw {
() => {
// Module: crate::ext::informational
// Provides: {"on_informational_raw"}
// Dependencies: {}
pub (crate) fn on_informational_raw < B , C > (req : & mut http :: Request < B > , callback : C) where C : OnInformationalCallback + Send + Sync + 'static , { req . extensions_mut () . insert (OnInformational (Arc :: new (callback))) ; }
};
}
