// Generated macro for RECV_TIME_BOUND (const)
macro_rules! DepcrateRECV_TIME_BOUND {
() => {
// Module: crate
// Provides: {"RECV_TIME_BOUND"}
// Dependencies: {}
# [doc = " The maximum amount of time that should be spent in `recvmsg()` calls per endpoint iteration"] # [doc = ""] # [doc = " 50us are chosen so that an endpoint iteration with a 50us sendmsg limit blocks"] # [doc = " the runtime for a maximum of about 100us."] # [doc = " Going much lower does not yield any noticeable difference, since a single `recvmmsg`"] # [doc = " batch of size 32 was observed to take 30us on some systems."] const RECV_TIME_BOUND : Duration = Duration :: from_micros (50) ;
};
}
