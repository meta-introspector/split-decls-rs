// Generated macro for headers_read_loop (function)
macro_rules! Depcrate_prompts_h3_headersheaders_read_loop {
() => {
// Module: crate::prompts::h3::headers
// Provides: {"headers_read_loop"}
// Dependencies: {}
fn headers_read_loop () -> InquireResult < Vec < quiche :: h3 :: Header > > { let mut headers = vec ! [] ; loop { let name = Text :: new ("field name:") . with_help_message ("type 'q!' to complete headers, or ESC to return to actions" ,) . prompt () ? ; if name == "q!" { break ; } let value = Text :: new ("field value:") . with_help_message (ESC_TO_RET) . prompt () ? ; headers . push (quiche :: h3 :: Header :: new (name . as_bytes () , value . as_bytes ())) ; } Ok (headers) }
};
}
