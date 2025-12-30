// Generated macro for impl_129 (impl)
macro_rules! Depcrate_flycheckimpl_129 {
() => {
// Module: crate::flycheck
// Provides: {"impl_129"}
// Dependencies: {}
impl CargoParser < CargoCheckMessage > for CargoCheckParser { fn from_line (& self , line : & str , error : & mut String) -> Option < CargoCheckMessage > { let mut deserializer = serde_json :: Deserializer :: from_str (line) ; deserializer . disable_recursion_limit () ; if let Ok (message) = JsonMessage :: deserialize (& mut deserializer) { return match message { JsonMessage :: Cargo (message) => match message { cargo_metadata :: Message :: CompilerArtifact (artifact) if ! artifact . fresh => { Some (CargoCheckMessage :: CompilerArtifact (artifact)) } cargo_metadata :: Message :: CompilerMessage (msg) => { Some (CargoCheckMessage :: Diagnostic { diagnostic : msg . message , package_id : Some (Arc :: new (msg . package_id)) , }) } _ => None , } , JsonMessage :: Rustc (message) => { Some (CargoCheckMessage :: Diagnostic { diagnostic : message , package_id : None }) } } ; } error . push_str (line) ; error . push ('\n') ; None } fn from_eof (& self) -> Option < CargoCheckMessage > { None } }
};
}
