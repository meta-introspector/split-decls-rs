// Generated macro for impl_62 (impl)
macro_rules! Depcrate_os_iocp_afdimpl_62 {
() => {
// Module: crate::os::iocp::afd
// Provides: {"impl_62"}
// Dependencies: {}
impl AfdPollMask { pub (crate) const RECEIVE : AfdPollMask = AfdPollMask (0x001) ; pub (crate) const RECEIVE_EXPEDITED : AfdPollMask = AfdPollMask (0x002) ; pub (crate) const SEND : AfdPollMask = AfdPollMask (0x004) ; pub (crate) const DISCONNECT : AfdPollMask = AfdPollMask (0x008) ; pub (crate) const ABORT : AfdPollMask = AfdPollMask (0x010) ; pub (crate) const LOCAL_CLOSE : AfdPollMask = AfdPollMask (0x020) ; pub (crate) const ACCEPT : AfdPollMask = AfdPollMask (0x080) ; pub (crate) const CONNECT_FAIL : AfdPollMask = AfdPollMask (0x100) ; # [doc = " Creates an empty mask."] pub (crate) const fn empty () -> AfdPollMask { AfdPollMask (0) } # [doc = " Checks if this mask contains the other mask."] pub (crate) fn intersects (self , other : AfdPollMask) -> bool { (self . 0 & other . 0) != 0 } # [doc = " Sets a flag."] pub (crate) fn set (& mut self , other : AfdPollMask , value : bool) { if value { * self |= other ; } else { self . 0 &= ! other . 0 ; } } }
};
}
