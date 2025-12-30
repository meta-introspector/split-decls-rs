// Generated macro for impl_144 (impl)
macro_rules! Depcrate_ipnet_serdeimpl_144 {
() => {
// Module: crate::ipnet_serde
// Provides: {"impl_144"}
// Dependencies: {}
impl Serialize for Ipv4Net { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer { if serializer . is_human_readable () { # [cfg (feature = "ser_as_str")] { let mut buf = heapless :: String :: < 18 > :: new () ; fmt :: write (& mut buf , format_args ! ("{self}")) . unwrap () ; serializer . serialize_str (& buf) } # [cfg (not (feature = "ser_as_str"))] serializer . collect_str (self) } else { let mut seq = serializer . serialize_tuple (5) ? ; for octet in & self . addr () . octets () { seq . serialize_element (octet) ? ; } seq . serialize_element (& self . prefix_len ()) ? ; seq . end () } } }
};
}
