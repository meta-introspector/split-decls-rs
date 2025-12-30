// Generated macro for spawn (function)
macro_rules! Depcrate_schedulerspawn {
() => {
// Module: crate::scheduler
// Provides: {"spawn"}
// Dependencies: {}
pub unsafe fn spawn (func : unsafe extern "C" fn (usize) , arg : usize , prio : Priority , stack_size : usize , selector : isize ,) -> TaskId { static CORE_COUNTER : AtomicU32 = AtomicU32 :: new (1) ; let core_id = if selector < 0 { CORE_COUNTER . fetch_add (1 , Ordering :: SeqCst) % get_processor_count () } else { selector as u32 } ; unsafe { PerCoreScheduler :: spawn (func , arg , prio , core_id , stack_size) } }
};
}
