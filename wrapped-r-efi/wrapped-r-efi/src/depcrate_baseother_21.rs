// Generated macro for other_21 (other)
macro_rules! Depcrate_baseother_21 {
() => {
// Module: crate::base
// Provides: {"other_21"}
// Dependencies: {}
# [doc = " IP Address"] # [doc = ""] # [doc = " A union type over the different IP addresses available. Alignment is always"] # [doc = " fixed to 4-bytes. Note that trailing bytes might be random, so no"] # [doc = " comparison functions are derived."] # [repr (C , align (4))] # [derive (Clone , Copy)] pub union IpAddress { pub addr : [u32 ; 4] , pub v4 : Ipv4Address , pub v6 : Ipv6Address , }
};
}
