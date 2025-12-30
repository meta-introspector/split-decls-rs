// Generated macro for PROT_NONE_FREE_LIST (static)
macro_rules! Depcrate_syscalls_mmanPROT_NONE_FREE_LIST {
() => {
// Module: crate::syscalls::mman
// Provides: {"PROT_NONE_FREE_LIST"}
// Dependencies: {}
static PROT_NONE_FREE_LIST : SpinMutex < FreeList < 16 > > = SpinMutex :: new (FreeList :: new ()) ;
};
}
