// Generated macro for uhyve_hypercall (function)
macro_rules! Depcrate_syscalls_interfaces_uhyveuhyve_hypercall {
() => {
// Module: crate::syscalls::interfaces::uhyve
// Provides: {"uhyve_hypercall"}
// Dependencies: {}
# [doc = " Perform a hypercall to the uhyve hypervisor"] # [inline] # [allow (unused_variables)] pub (crate) fn uhyve_hypercall (hypercall : Hypercall < '_ >) { let ptr = HypercallAddress :: from (& hypercall) as u16 ; let data = hypercall_data (& hypercall) ; # [cfg (target_arch = "x86_64")] unsafe { use x86_64 :: instructions :: port :: Port ; let data = u32 :: try_from (data) . expect ("Hypercall data must lie in the first 4GiB of memory") ; Port :: new (ptr) . write (data) ; } # [cfg (target_arch = "aarch64")] unsafe { use core :: arch :: asm ; asm ! ("str x8, [{ptr}]" , ptr = in (reg) u64 :: from (ptr) , in ("x8") data , options (nostack) ,) ; } # [cfg (target_arch = "riscv64")] todo ! () }
};
}
