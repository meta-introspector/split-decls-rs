// Generated macro for impl_26 (impl)
macro_rules! Depcrate_buildimpl_26 {
() => {
// Module: crate::build
// Provides: {"impl_26"}
// Dependencies: {}
impl < 'a > UserinfoBuilder < 'a > { # [doc = " Decomposes the userinfo into `user` and `password`."] # [must_use] fn to_user_password (& self) -> Option < (& 'a str , Option < & 'a str >) > { match & self . 0 { UserinfoRepr :: None => None , UserinfoRepr :: Direct (s) => match find_split (s , b':') { None => Some ((s , None)) , Some ((user , password)) => Some ((user , Some (password))) , } , UserinfoRepr :: UserPass (user , password) => Some ((* user , * password)) , } } }
};
}
