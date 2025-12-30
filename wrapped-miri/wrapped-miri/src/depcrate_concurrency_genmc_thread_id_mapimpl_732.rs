// Generated macro for impl_732 (impl)
macro_rules! Depcrate_concurrency_genmc_thread_id_mapimpl_732 {
() => {
// Module: crate::concurrency::genmc::thread_id_map
// Provides: {"impl_732"}
// Dependencies: {}
impl ThreadIdMap { pub fn reset (& mut self) { self . miri_to_genmc . clear () ; self . miri_to_genmc . insert (ThreadId :: MAIN_THREAD , GENMC_MAIN_THREAD_ID) ; self . genmc_to_miri . clear () ; self . genmc_to_miri . push (ThreadId :: MAIN_THREAD) ; } # [must_use] # [doc = " Add a new Miri thread to the mapping and dispense a new thread ID for GenMC to use."] pub fn add_thread (& mut self , thread_id : ThreadId) -> i32 { let next_thread_id = self . genmc_to_miri . len () ; let genmc_tid = next_thread_id . try_into () . unwrap () ; self . miri_to_genmc . insert (thread_id , genmc_tid) ; self . genmc_to_miri . push (thread_id) ; genmc_tid } # [must_use] # [doc = " Try to get the GenMC thread ID corresponding to a given Miri `ThreadId`."] # [doc = " Panics if there is no mapping for the given `ThreadId`."] pub fn get_genmc_tid (& self , thread_id : ThreadId) -> i32 { * self . miri_to_genmc . get (& thread_id) . unwrap () } # [must_use] # [doc = " Get the Miri `ThreadId` corresponding to a given GenMC thread id."] # [doc = " Panics if the given thread id isn't valid."] pub fn get_miri_tid (& self , genmc_tid : i32) -> ThreadId { let index : usize = genmc_tid . try_into () . unwrap () ; self . genmc_to_miri . get (index) . copied () . expect ("A thread id returned from GenMC should exist.") } }
};
}
