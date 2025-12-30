// Generated macro for CORE_HLT_STATE (static)
macro_rules! Depcrate_schedulerCORE_HLT_STATE {
() => {
// Module: crate::scheduler
// Provides: {"CORE_HLT_STATE"}
// Dependencies: {}
# [cfg (all (target_arch = "x86_64" , feature = "smp"))] static CORE_HLT_STATE : SpinMutex < Vec < & AtomicBool > > = SpinMutex :: new (Vec :: new ()) ;
};
}
