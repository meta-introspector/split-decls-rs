// Generated macro for NtGetTickCount64 (function)
macro_rules! Depcrate_ntexapiNtGetTickCount64 {
() => {
// Module: crate::ntexapi
// Provides: {"NtGetTickCount64"}
// Dependencies: {}
# [inline] pub unsafe fn NtGetTickCount64 () -> ULONGLONG { let mut tick_count : ULARGE_INTEGER = MaybeUninit :: zeroed () . assume_init () ; # [cfg (any (target_arch = "x86_64" , target_arch = "aarch64"))] { * tick_count . QuadPart_mut () = read_volatile (addr_of ! ((* USER_SHARED_DATA) . u . TickCountQuad)) ; } # [cfg (target_arch = "x86")] { loop { tick_count . s_mut () . HighPart = read_volatile (& (* USER_SHARED_DATA) . u . TickCount . High1Time) as u32 ; tick_count . s_mut () . LowPart = read_volatile (& (* USER_SHARED_DATA) . u . TickCount . LowPart) ; if tick_count . s () . HighPart == read_volatile (& (* USER_SHARED_DATA) . u . TickCount . High2Time) as u32 { break ; } spin_loop () ; } } (UInt32x32To64 (tick_count . s () . LowPart , (* USER_SHARED_DATA) . TickCountMultiplier) >> 24) + (UInt32x32To64 (tick_count . s () . HighPart as u32 , (* USER_SHARED_DATA) . TickCountMultiplier ,) << 8) }
};
}
