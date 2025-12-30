// Generated macro for custom_subprotocol (function)
macro_rules! Depcrate_runtime_test_utilscustom_subprotocol {
() => {
// Module: crate::runtime::test_utils
// Provides: {"custom_subprotocol"}
// Dependencies: {}
pub (crate) fn custom_subprotocol () -> & 'static AnyProtocol { static REGISTER_CUSTOM_SUBPROTOCOL : Once = Once :: new () ; REGISTER_CUSTOM_SUBPROTOCOL . call_once (| | { let super_proto = custom_protocol () ; let mut builder = ProtocolBuilder :: new (& c ("CustomSubProtocol")) . unwrap () ; builder . add_protocol (super_proto) ; builder . add_method_description :: < (u32 ,) , u32 > (sel ! (calculateFoo :) , true) ; builder . register () ; }) ; AnyProtocol :: get (& c ("CustomSubProtocol")) . unwrap () }
};
}
