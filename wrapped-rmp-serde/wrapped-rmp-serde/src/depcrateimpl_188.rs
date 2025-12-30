// Generated macro for impl_188 (impl)
macro_rules! Depcrateimpl_188 {
() => {
// Module: crate
// Provides: {"impl_188"}
// Dependencies: {}
impl de :: Visitor < '_ > for RawVisitor { type Value = Raw ; # [cold] fn expecting (& self , fmt : & mut Formatter < '_ >) -> Result < () , fmt :: Error > { "string or bytes" . fmt (fmt) } # [inline] fn visit_string < E > (self , v : String) -> Result < Self :: Value , E > { Ok (Raw { s : Ok (v) }) } # [inline] fn visit_str < E > (self , v : & str) -> Result < Self :: Value , E > where E : de :: Error { Ok (Raw { s : Ok (v . into ()) }) } # [inline] fn visit_bytes < E > (self , v : & [u8]) -> Result < Self :: Value , E > where E : de :: Error { let s = match str :: from_utf8 (v) { Ok (s) => Ok (s . into ()) , Err (err) => Err ((v . into () , err)) , } ; Ok (Raw { s }) } # [inline] fn visit_byte_buf < E > (self , v : Vec < u8 >) -> Result < Self :: Value , E > where E : de :: Error { let s = match String :: from_utf8 (v) { Ok (s) => Ok (s) , Err (err) => { let e = err . utf8_error () ; Err ((err . into_bytes () , e)) } } ; Ok (Raw { s }) } }
};
}
