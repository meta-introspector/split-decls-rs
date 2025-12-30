// Generated macro for impl_194 (impl)
macro_rules! Depcrateimpl_194 {
() => {
// Module: crate
// Provides: {"impl_194"}
// Dependencies: {}
impl < 'de > de :: Visitor < 'de > for RawRefVisitor { type Value = RawRef < 'de > ; # [cold] fn expecting (& self , fmt : & mut Formatter < '_ >) -> Result < () , fmt :: Error > { "string or bytes" . fmt (fmt) } # [inline] fn visit_borrowed_str < E > (self , v : & 'de str) -> Result < Self :: Value , E > where E : de :: Error { Ok (RawRef { s : Ok (v) }) } # [inline] fn visit_borrowed_bytes < E > (self , v : & 'de [u8]) -> Result < Self :: Value , E > where E : de :: Error { let s = match str :: from_utf8 (v) { Ok (s) => Ok (s) , Err (err) => Err ((v , err)) , } ; Ok (RawRef { s }) } }
};
}
