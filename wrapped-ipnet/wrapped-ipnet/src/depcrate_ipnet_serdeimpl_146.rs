// Generated macro for impl_146 (impl)
macro_rules! Depcrate_ipnet_serdeimpl_146 {
() => {
// Module: crate::ipnet_serde
// Provides: {"impl_146"}
// Dependencies: {}
impl Serialize for Ipv6Net { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer { if serializer . is_human_readable () { # [cfg (feature = "ser_as_str")] { let mut buf = heapless :: String :: < 43 > :: new () ; fmt :: write (& mut buf , format_args ! ("{self}")) . unwrap () ; serializer . serialize_str (& buf) } # [cfg (not (feature = "ser_as_str"))] serializer . collect_str (self) } else { let mut seq = serializer . serialize_tuple (17) ? ; for octet in & self . addr () . octets () { seq . serialize_element (octet) ? ; } seq . serialize_element (& self . prefix_len ()) ? ; seq . end () } } }
};
}
