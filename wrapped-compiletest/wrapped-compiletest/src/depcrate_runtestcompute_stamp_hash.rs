// Generated macro for compute_stamp_hash (function)
macro_rules! Depcrate_runtestcompute_stamp_hash {
() => {
// Module: crate::runtest
// Provides: {"compute_stamp_hash"}
// Dependencies: {}
pub fn compute_stamp_hash (config : & Config) -> String { let mut hash = DefaultHasher :: new () ; config . stage_id . hash (& mut hash) ; config . run . hash (& mut hash) ; config . edition . hash (& mut hash) ; match config . debugger { Some (Debugger :: Cdb) => { config . cdb . hash (& mut hash) ; } Some (Debugger :: Gdb) => { config . gdb . hash (& mut hash) ; env :: var_os ("PATH") . hash (& mut hash) ; env :: var_os ("PYTHONPATH") . hash (& mut hash) ; } Some (Debugger :: Lldb) => { config . python . hash (& mut hash) ; config . lldb_python_dir . hash (& mut hash) ; env :: var_os ("PATH") . hash (& mut hash) ; env :: var_os ("PYTHONPATH") . hash (& mut hash) ; } None => { } } if config . mode == TestMode :: Ui { config . force_pass_mode . hash (& mut hash) ; } format ! ("{:x}" , hash . finish ()) }
};
}
