// Generated macro for impl_25 (impl)
macro_rules! Depcrate_buildimpl_25 {
() => {
// Module: crate::build
// Provides: {"impl_25"}
// Dependencies: {}
impl fmt :: Debug for UserinfoBuilder < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut debug = f . debug_struct ("UserinfoBuilder") ; if let Some ((user , password)) = self . to_user_password () { debug . field ("user" , & user) ; if matches ! (password , None | Some ("")) { debug . field ("password" , & password) ; } else { debug . field ("password" , & Some (Censored)) ; } } debug . finish () } }
};
}
