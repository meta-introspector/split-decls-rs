// Generated macro for init_with_str (function)
macro_rules! Depcrate_stringinit_with_str {
() => {
// Module: crate::string
// Provides: {"init_with_str"}
// Dependencies: {}
unsafe fn init_with_str < T : Message > (obj : Allocated < T > , string : & str) -> Retained < T > { let bytes : * const c_void = string . as_ptr () . cast () ; unsafe { msg_send ! [obj , initWithBytes : bytes , length : string . len () , encoding : UTF8_ENCODING ,] } }
};
}
