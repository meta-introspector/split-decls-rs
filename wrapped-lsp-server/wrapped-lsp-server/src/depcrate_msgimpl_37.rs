// Generated macro for impl_37 (impl)
macro_rules! Depcrate_msgimpl_37 {
() => {
// Module: crate::msg
// Provides: {"impl_37"}
// Dependencies: {}
impl Notification { pub fn new (method : String , params : impl serde :: Serialize) -> Notification { Notification { method , params : serde_json :: to_value (params) . unwrap () } } pub fn extract < P : DeserializeOwned > (self , method : & str ,) -> Result < P , ExtractError < Notification > > { if self . method != method { return Err (ExtractError :: MethodMismatch (self)) ; } match serde_json :: from_value (self . params) { Ok (params) => Ok (params) , Err (error) => Err (ExtractError :: JsonError { method : self . method , error }) , } } pub (crate) fn is_exit (& self) -> bool { self . method == "exit" } pub (crate) fn is_initialized (& self) -> bool { self . method == "initialized" } }
};
}
