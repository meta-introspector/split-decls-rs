// Generated macro for get_query_result (function)
macro_rules! Depcrate_executor_networkget_query_result {
() => {
// Module: crate::executor::network
// Provides: {"get_query_result"}
// Dependencies: {}
# [cfg (feature = "dns")] pub (crate) async fn get_query_result (query : QueryHandle) -> io :: Result < Vec < IpAddress > > { future :: poll_fn (| cx | { let Some (mut guard) = NIC . try_lock () else { cx . waker () . wake_by_ref () ; return Poll :: Pending ; } ; let nic = guard . as_nic_mut () . unwrap () ; let socket = nic . get_mut_dns_socket () ? ; match socket . get_query_result (query) { Ok (addrs) => { let mut ips = Vec :: new () ; for x in & addrs { ips . push (* x) ; } Poll :: Ready (Ok (ips)) } Err (GetQueryResultError :: Pending) => { socket . register_query_waker (query , cx . waker ()) ; Poll :: Pending } Err (e) => { warn ! ("DNS query failed: {e:?}") ; Poll :: Ready (Err (Errno :: Noent)) } } }) . await }
};
}
