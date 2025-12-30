// Generated macro for userdata_get (function)
macro_rules! Depcrate_userdatauserdata_get {
() => {
// Module: crate::userdata
// Provides: {"userdata_get"}
// Dependencies: {}
pub (crate) fn userdata_get () -> Result < * mut c_void , UserdataError > { USERDATA . try_with (| userdata | { userdata . try_borrow_mut () . map_or_else (| _ | Err (UserdataError :: AlreadyBorrowed) , | v | match v . last () { Some (u) => Ok (u . userdata) , None => Err (UserdataError :: EmptyStack) , } ,) }) . unwrap_or (Err (UserdataError :: AccessError)) }
};
}
