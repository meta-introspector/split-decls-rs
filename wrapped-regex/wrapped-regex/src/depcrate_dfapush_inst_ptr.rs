// Generated macro for push_inst_ptr (function)
macro_rules! Depcrate_dfapush_inst_ptr {
() => {
// Module: crate::dfa
// Provides: {"push_inst_ptr"}
// Dependencies: {}
# [doc = " Adds ip to data using delta encoding with respect to prev."] # [doc = ""] # [doc = " After completion, `data` will contain `ip` and `prev` will be set to `ip`."] fn push_inst_ptr (data : & mut Vec < u8 > , prev : & mut InstPtr , ip : InstPtr) { let delta = (ip as i32) - (* prev as i32) ; write_vari32 (data , delta) ; * prev = ip ; }
};
}
