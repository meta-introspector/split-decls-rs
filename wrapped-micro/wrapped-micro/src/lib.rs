// Generated wrapped crate with macro-based items

include!("depcrate_benchesuse_3.rs");
include!("depcrate_benchesbench_sched_one_thread.rs");
include!("depcrate_benchesmemcpy_builtin.rs");
include!("depcrate_benchesmemset_rust.rs");
include!("depcrate_benchesuse_4.rs");
include!("depcrate_benchesbench_mem.rs");
include!("depcrate_benchesbench_sched_two_threads.rs");
include!("depcrate_benchesuse_5.rs");
include!("depcrate_benchesget_timestamp.rs");
include!("depcrate_benchesnr_runs.rs");
include!("depcrate_benchesmemset_builtin.rs");
include!("depcrate_benchesmemcpy_rust.rs");
include!("depcrateuse_21.rs");
include!("depcrate_benchesother_12.rs");
include!("depcratebenches.rs");
include!("depcrate_benchesother_7.rs");
include!("depcrate_benchesbench_syscall.rs");
include!("depcratemain.rs");
include!("depcrate_benchesuse_6.rs");
include!("depcrateuse_1.rs");
include!("modcrate_benches.rs");
include!("modcrate.rs");

// Execute all items
pub fn execute_all() {
    Modcrate_benches!();
    Modcrate!();
}
