// Generated macro for impl_9 (impl)
macro_rules! Depcrateimpl_9 {
() => {
// Module: crate
// Provides: {"impl_9"}
// Dependencies: {}
impl Versions { fn get (& self , name : & str) -> Result < & str , Error > { Ok (match name { "msrv" => & self . msrv , "stable" => & self . stable , "nightly" => & self . nightly , _ => self . build_rs . get (name) . ok_or (Error :: UnrecognizedToolchain (name . to_string ())) . map (| value | value . as_str () . unwrap ()) ? , }) } }
};
}
