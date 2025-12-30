// Generated macro for pseudo_headers (function)
macro_rules! Depcrate_prompts_h3_headerspseudo_headers {
() => {
// Module: crate::prompts::h3::headers
// Provides: {"pseudo_headers"}
// Dependencies: {}
fn pseudo_headers (host_port : & str) -> InquireResult < Vec < quiche :: h3 :: Header > > { let method = Text :: new ("method:") . with_autocomplete (& method_suggester) . with_default ("GET") . with_help_message (ESC_TO_RET) . prompt () ? ; let help = format ! ("Press enter/return for default ({host_port}") ; let authority = Text :: new ("authority:") . with_default (host_port) . with_help_message (& help) . prompt () ? ; let path = Text :: new ("path:") . with_default ("/") . prompt () ? ; let scheme = Text :: new ("scheme:") . with_default ("https") . with_help_message (ESC_TO_RET) . prompt () ? ; Ok (vec ! [quiche :: h3 :: Header :: new (b":method" , method . as_bytes ()) , quiche :: h3 :: Header :: new (b":authority" , authority . as_bytes ()) , quiche :: h3 :: Header :: new (b":path" , path . as_bytes ()) , quiche :: h3 :: Header :: new (b":scheme" , scheme . as_bytes ()) ,]) }
};
}
