// SRC: ../rust/library/std/src/net/ip_addr/tests.rs
use crate::net::Ipv4Addr;
use crate::net::test::{sa4, tsa};

#[test]
fn to_socket_addr_socketaddr() {
    let a = sa4(Ipv4Addr::new(77, 88, 21, 11), 12345);
    assert_eq!(Ok(vec![a]), tsa(a));
}
