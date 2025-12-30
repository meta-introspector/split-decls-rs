// Generated macro for impl_16 (impl)
macro_rules! Depcrate_memory_usageimpl_16 {
() => {
// Module: crate::memory_usage
// Provides: {"impl_16"}
// Dependencies: {}
impl MemoryUsage { pub fn now () -> MemoryUsage { cfg_if ! { if # [cfg (all (feature = "jemalloc" , not (target_env = "msvc")))] { jemalloc_ctl :: epoch :: advance () . unwrap () ; MemoryUsage { allocated : Bytes (jemalloc_ctl :: stats :: allocated :: read () . unwrap () as isize) , } } else if # [cfg (all (target_os = "linux" , target_env = "gnu"))] { memusage_linux () } else if # [cfg (windows)] { use windows_sys :: Win32 :: System :: { Threading ::*, ProcessStatus ::* } ; use std :: mem :: MaybeUninit ; let proc = unsafe { GetCurrentProcess () } ; let mut mem_counters = MaybeUninit :: uninit () ; let cb = size_of ::< PROCESS_MEMORY_COUNTERS > () ; let ret = unsafe { GetProcessMemoryInfo (proc , mem_counters . as_mut_ptr () , cb as u32) } ; assert ! (ret != 0) ; let usage = unsafe { mem_counters . assume_init () . PagefileUsage } ; MemoryUsage { allocated : Bytes (usage as isize) } } else { MemoryUsage { allocated : Bytes (0) } } } } }
};
}
