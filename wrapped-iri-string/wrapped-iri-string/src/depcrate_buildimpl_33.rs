// Generated macro for impl_33 (impl)
macro_rules! Depcrate_buildimpl_33 {
() => {
// Module: crate::build
// Provides: {"impl_33"}
// Dependencies: {}
impl AuthorityBuilder < '_ > { # [doc = " Writes the authority to the given formatter."] fn fmt_write_to < S : Spec > (& self , f : & mut fmt :: Formatter < '_ > , normalize : bool) -> fmt :: Result { match & self . userinfo . 0 { UserinfoRepr :: None => { } UserinfoRepr :: Direct (userinfo) => { if normalize { PctCaseNormalized :: < S > :: new (userinfo) . fmt (f) ? ; } else { userinfo . fmt (f) ? ; } f . write_char ('@') ? ; } UserinfoRepr :: UserPass (user , password) => { if normalize { PctCaseNormalized :: < S > :: new (user) . fmt (f) ? ; } else { f . write_str (user) ? ; } if let Some (password) = password { f . write_char (':') ? ; if normalize { PctCaseNormalized :: < S > :: new (password) . fmt (f) ? ; } else { password . fmt (f) ? ; } } f . write_char ('@') ? ; } } match self . host { HostRepr :: String (host) => { if normalize { normalize :: normalize_host_port :: < S > (f , host) ? ; } else { f . write_str (host) ? ; } } # [cfg (feature = "std")] HostRepr :: IpAddr (ipaddr) => match ipaddr { std :: net :: IpAddr :: V4 (v) => v . fmt (f) ? , std :: net :: IpAddr :: V6 (v) => write ! (f , "[{v}]") ? , } , } match self . port . 0 { PortBuilderRepr :: Empty => { } PortBuilderRepr :: Integer (v) => write ! (f , ":{v}") ? , PortBuilderRepr :: String (v) => { if ! (v . is_empty () && normalize) { write ! (f , ":{v}") ? ; } } } Ok (()) } }
};
}
