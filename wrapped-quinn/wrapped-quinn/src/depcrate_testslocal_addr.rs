// Generated macro for local_addr (function)
macro_rules! Depcrate_testslocal_addr {
() => {
// Module: crate::tests
// Provides: {"local_addr"}
// Dependencies: {}
# [test] fn local_addr () { let socket = UdpSocket :: bind ((Ipv6Addr :: LOCALHOST , 0)) . unwrap () ; let addr = socket . local_addr () . unwrap () ; let runtime = rt_basic () ; let ep = { let _guard = runtime . enter () ; Endpoint :: new (Default :: default () , None , socket , Arc :: new (TokioRuntime)) . unwrap () } ; assert_eq ! (addr , ep . local_addr () . expect ("Could not obtain our local endpoint")) ; }
};
}
