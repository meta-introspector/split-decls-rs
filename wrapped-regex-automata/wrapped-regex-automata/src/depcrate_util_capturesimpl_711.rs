// Generated macro for impl_711 (impl)
macro_rules! Depcrate_util_capturesimpl_711 {
() => {
// Module: crate::util::captures
// Provides: {"impl_711"}
// Dependencies: {}
impl core :: fmt :: Debug for Captures { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { let mut dstruct = f . debug_struct ("Captures") ; dstruct . field ("pid" , & self . pid) ; if let Some (pid) = self . pid { dstruct . field ("spans" , & CapturesDebugMap { pid , caps : self }) ; } dstruct . finish () } }
};
}
