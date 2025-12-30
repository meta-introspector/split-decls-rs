// Generated macro for impl_34 (impl)
macro_rules! Depcrate_msgimpl_34 {
() => {
// Module: crate::msg
// Provides: {"impl_34"}
// Dependencies: {}
impl Message { pub fn read (r : & mut impl BufRead) -> io :: Result < Option < Message > > { Message :: _read (r) } fn _read (r : & mut dyn BufRead) -> io :: Result < Option < Message > > { let text = match read_msg_text (r) ? { None => return Ok (None) , Some (text) => text , } ; let msg = match serde_json :: from_str (& text) { Ok (msg) => msg , Err (e) => { return Err (invalid_data ! ("malformed LSP payload `{e:?}`: {text:?}")) ; } } ; Ok (Some (msg)) } pub fn write (& self , w : & mut impl Write) -> io :: Result < () > { self . _write (w) } fn _write (& self , w : & mut dyn Write) -> io :: Result < () > { # [derive (Serialize)] struct JsonRpc < 'a > { jsonrpc : & 'static str , # [serde (flatten)] msg : & 'a Message , } let text = serde_json :: to_string (& JsonRpc { jsonrpc : "2.0" , msg : self }) ? ; write_msg_text (w , & text) } }
};
}
