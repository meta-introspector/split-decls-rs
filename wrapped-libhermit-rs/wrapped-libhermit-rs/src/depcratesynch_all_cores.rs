// Generated macro for synch_all_cores (function)
macro_rules! Depcratesynch_all_cores {
() => {
// Module: crate
// Provides: {"synch_all_cores"}
// Dependencies: {}
# [cfg (feature = "smp")] fn synch_all_cores () { static CORE_COUNTER : AtomicU32 = AtomicU32 :: new (0) ; CORE_COUNTER . fetch_add (1 , Ordering :: SeqCst) ; let possible_cpus = kernel :: get_possible_cpus () ; while CORE_COUNTER . load (Ordering :: SeqCst) != possible_cpus { spin_loop () ; } }
};
}
