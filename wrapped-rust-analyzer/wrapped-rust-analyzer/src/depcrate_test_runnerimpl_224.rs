// Generated macro for impl_224 (impl)
macro_rules! Depcrate_test_runnerimpl_224 {
() => {
// Module: crate::test_runner
// Provides: {"impl_224"}
// Dependencies: {}
impl CargoParser < CargoTestMessage > for CargoTestOutputParser { fn from_line (& self , line : & str , _error : & mut String) -> Option < CargoTestMessage > { let mut deserializer = serde_json :: Deserializer :: from_str (line) ; deserializer . disable_recursion_limit () ; Some (CargoTestMessage { target : self . target . clone () , output : if let Ok (message) = CargoTestOutput :: deserialize (& mut deserializer) { message } else { CargoTestOutput :: Custom { text : line . to_owned () } } , }) } fn from_eof (& self) -> Option < CargoTestMessage > { Some (CargoTestMessage { target : self . target . clone () , output : CargoTestOutput :: Finished }) } }
};
}
