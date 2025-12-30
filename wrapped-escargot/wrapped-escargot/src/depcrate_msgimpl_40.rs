// Generated macro for impl_40 (impl)
macro_rules! Depcrate_msgimpl_40 {
() => {
// Module: crate::msg
// Provides: {"impl_40"}
// Dependencies: {}
impl Message { # [doc = " Deserialize the message."] pub fn decode (& self) -> CargoResult < format :: Message < '_ > > { self . decode_custom () } # [doc = " Deserialize the message."] pub fn decode_custom < 'a , T > (& 'a self) -> CargoResult < T > where T : serde :: Deserialize < 'a > , { let data = serde_json :: from_str (self . 0 . as_str ()) . map_err (| e | CargoError :: new (ErrorKind :: InvalidOutput) . set_cause (e)) ? ; Ok (data) } }
};
}
