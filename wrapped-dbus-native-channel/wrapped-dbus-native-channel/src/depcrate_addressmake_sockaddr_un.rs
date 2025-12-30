// Generated macro for make_sockaddr_un (function)
macro_rules! Depcrate_addressmake_sockaddr_un {
() => {
// Module: crate::address
// Provides: {"make_sockaddr_un"}
// Dependencies: {}
fn make_sockaddr_un (start : usize , s : & str) -> Result < libc :: sockaddr_un , Box < dyn std :: error :: Error > > { let bytes = s . as_bytes () ; let mut r = libc :: sockaddr_un { sun_family : libc :: AF_UNIX as libc :: sa_family_t , sun_path : [0 ; 108] , } ; if start + bytes . len () + 1 >= r . sun_path . len () { Err ("Address too long") ? } for (i , & x) in bytes . into_iter () . enumerate () { r . sun_path [i + start] = x as libc :: c_char ; } Ok (r) }
};
}
