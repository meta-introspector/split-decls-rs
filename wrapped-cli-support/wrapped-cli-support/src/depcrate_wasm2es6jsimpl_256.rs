// Generated macro for impl_256 (impl)
macro_rules! Depcrate_wasm2es6jsimpl_256 {
() => {
// Module: crate::wasm2es6js
// Provides: {"impl_256"}
// Dependencies: {}
impl Config { pub fn new () -> Config { Config { base64 : false , fetch_path : None , } } pub fn base64 (& mut self , base64 : bool) -> & mut Self { self . base64 = base64 ; self } pub fn fetch (& mut self , path : Option < String >) -> & mut Self { self . fetch_path = path ; self } pub fn generate (& mut self , wasm : & [u8]) -> Result < Output , Error > { if ! self . base64 && self . fetch_path . is_none () { bail ! ("one of --base64 or --fetch is required") ; } let module = Module :: from_buffer (wasm) ? ; Ok (Output { module , base64 : self . base64 , fetch_path : self . fetch_path . clone () , }) } }
};
}
