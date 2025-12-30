// Generated macro for custom_protocol (function)
macro_rules! Depcrate_runtime_test_utilscustom_protocol {
() => {
// Module: crate::runtime::test_utils
// Provides: {"custom_protocol"}
// Dependencies: {}
pub (crate) fn custom_protocol () -> & 'static AnyProtocol { static REGISTER_CUSTOM_PROTOCOL : Once = Once :: new () ; REGISTER_CUSTOM_PROTOCOL . call_once (| | { let mut builder = ProtocolBuilder :: new (& c ("CustomProtocol")) . unwrap () ; builder . add_method_description :: < (i32 ,) , () > (sel ! (setBar :) , true) ; builder . add_method_description :: < () , * const c_char > (sel ! (getName) , false) ; builder . add_class_method_description :: < (i32 , i32) , i32 > (sel ! (addNumber : toNumber :) , true) ; builder . register () ; }) ; AnyProtocol :: get (& c ("CustomProtocol")) . unwrap () }
};
}
