// Generated macro for impl_1641 (impl)
macro_rules! Depcrate_syscalls_socket_addrinfoimpl_1641 {
() => {
// Module: crate::syscalls::socket::addrinfo
// Provides: {"impl_1641"}
// Dependencies: {}
impl Extend < addrinfo > for addrinfoList { fn extend < T : IntoIterator < Item = addrinfo > > (& mut self , iter : T) { let mut place = & mut self . 0 ; while let Some (some) = place { place = & mut some . ai_next ; } for addrinfo in iter { assert ! (addrinfo . ai_next . is_none ()) ; let addrinfo = place . insert (Box :: new (addrinfo)) ; place = & mut addrinfo . ai_next ; } } }
};
}
