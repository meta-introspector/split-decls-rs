// Generated macro for macro_12 (macro)
macro_rules! Depcratemacro_12 {
() => {
// Module: crate
// Provides: {"macro_12"}
// Dependencies: {}
cfg_if ! { if # [cfg (target_os = "linux")] { fn valgrind_without_aslr (arch : & str) -> Command { let mut cmd = Command :: new ("setarch") ; cmd . arg (arch) . arg ("-R") . arg ("valgrind") ; cmd } } else if # [cfg (target_os = "freebsd")] { fn valgrind_without_aslr (_arch : & str) -> Command { let mut cmd = Command :: new ("proccontrol") ; cmd . arg ("-m") . arg ("aslr") . arg ("-s") . arg ("disable") ; cmd } } else { fn valgrind_without_aslr (_arch : & str) -> Command { basic_valgrind () } } }
};
}
