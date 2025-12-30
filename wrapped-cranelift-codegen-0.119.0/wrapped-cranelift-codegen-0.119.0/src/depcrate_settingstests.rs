// Generated macro for tests (module)
macro_rules! Depcrate_settingstests {
() => {
// Module: crate::settings
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: Configurable ; use super :: SetError :: * ; use super :: { builder , Flags } ; use alloc :: string :: ToString ; # [test] fn display_default () { let b = builder () ; let f = Flags :: new (b) ; let actual = f . to_string () ; let expected = r#"[shared]
regalloc_algorithm = "backtracking"
opt_level = "none"
tls_model = "none"
stack_switch_model = "none"
libcall_call_conv = "isa_default"
probestack_size_log2 = 12
probestack_strategy = "outline"
bb_padding_log2_minus_one = 0
log2_min_function_alignment = 0
regalloc_checker = false
regalloc_verbose_logs = false
enable_alias_analysis = true
enable_verifier = true
enable_pcc = false
is_pic = false
use_colocated_libcalls = false
enable_float = true
enable_nan_canonicalization = false
enable_pinned_reg = false
enable_atomics = true
enable_safepoints = false
enable_llvm_abi_extensions = false
enable_multi_ret_implicit_sret = false
unwind_info = true
preserve_frame_pointers = false
machine_code_cfg_info = false
enable_probestack = false
enable_jump_tables = true
enable_heap_access_spectre_mitigation = true
enable_table_access_spectre_mitigation = true
enable_incremental_compilation_cache_checks = false
"# ; if actual != expected { panic ! ("Default settings do not match expectations:\n\n{}" , similar :: TextDiff :: from_lines (expected , & actual) . unified_diff () . header ("expected" , "actual")) ; } assert_eq ! (f . opt_level () , super :: OptLevel :: None) ; } # [test] fn modify_bool () { let mut b = builder () ; assert_eq ! (b . enable ("not_there") , Err (BadName ("not_there" . to_string ()))) ; assert_eq ! (b . enable ("enable_atomics") , Ok (())) ; assert_eq ! (b . set ("enable_atomics" , "false") , Ok (())) ; let f = Flags :: new (b) ; assert_eq ! (f . enable_atomics () , false) ; } # [test] fn modify_string () { let mut b = builder () ; assert_eq ! (b . set ("not_there" , "true") , Err (BadName ("not_there" . to_string ()))) ; assert_eq ! (b . set ("enable_atomics" , "") , Err (BadValue ("bool" . to_string ()))) ; assert_eq ! (b . set ("enable_atomics" , "best") , Err (BadValue ("bool" . to_string ()))) ; assert_eq ! (b . set ("opt_level" , "true") , Err (BadValue ("any among none, speed, speed_and_size" . to_string ()))) ; assert_eq ! (b . set ("opt_level" , "speed") , Ok (())) ; assert_eq ! (b . set ("enable_atomics" , "0") , Ok (())) ; let f = Flags :: new (b) ; assert_eq ! (f . enable_atomics () , false) ; assert_eq ! (f . opt_level () , super :: OptLevel :: Speed) ; } }
};
}
