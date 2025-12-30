// Generated macro for NoProxy (struct)
macro_rules! Depcrate_client_proxy_matcherNoProxy {
() => {
// Module: crate::client::proxy::matcher
// Provides: {"NoProxy"}
// Dependencies: {}
# [doc = " A filter for proxy matchers."] # [doc = ""] # [doc = " This type is based off the `NO_PROXY` rules used by curl."] # [derive (Clone , Debug , Default)] struct NoProxy { ips : IpMatcher , domains : DomainMatcher , }
};
}
