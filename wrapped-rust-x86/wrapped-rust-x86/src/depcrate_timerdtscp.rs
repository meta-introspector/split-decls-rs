// Generated macro for rdtscp (function)
macro_rules! Depcrate_timerdtscp {
() => {
// Module: crate::time
// Provides: {"rdtscp"}
// Dependencies: {}
# [doc = " Read the time stamp counter."] # [doc = ""] # [doc = " The RDTSCP instruction waits until all previous instructions have been"] # [doc = " executed before reading the counter. However, subsequent instructions may"] # [doc = " begin execution before the read operation is performed."] # [doc = ""] # [doc = " Volatile is used here because the function may be used to act as an"] # [doc = " instruction barrier."] # [doc = ""] # [doc = " # Returns"] # [doc = " - The current time stamp counter value of the CPU as a `u64`."] # [doc = " - The contents of `IA32_TSC_AUX` on that particular core. This is an OS"] # [doc = "   defined value. For example, Linux writes `numa_id << 12 | core_id` into"] # [doc = "   it. See also [`crate::rdpid`]."] # [doc = ""] # [doc = " # Note"] # [doc = " One can use `core::arch::x86_64::__rdtscp` from the Rust core library as"] # [doc = " well. We don't rely on it because it only returns the time-stamp counter of"] # [doc = " rdtscp and not the contents of `IA32_TSC_AUX`."] # [doc = ""] # [doc = " # Safety"] # [doc = " * Causes a GP fault if the TSD flag in register CR4 is set and the CPL is"] # [doc = "   greater than 0."] pub unsafe fn rdtscp () -> (u64 , u32) { let eax : u32 ; let ecx : u32 ; let edx : u32 ; asm ! ("rdtscp" , lateout ("eax") eax , lateout ("ecx") ecx , lateout ("edx") edx , options (nomem , nostack)) ; let counter : u64 = (edx as u64) << 32 | eax as u64 ; (counter , ecx) }
};
}
