// Generated macro for declare_deprecated_id (macro)
macro_rules! Depcratedeclare_deprecated_id {
() => {
// Module: crate
// Provides: {"declare_deprecated_id"}
// Dependencies: {}
# [doc = " Same as [`declare_id`] except that it reports that this ID has been deprecated."] # [cfg (feature = "decode")] # [macro_export] macro_rules ! declare_deprecated_id { ($ address : expr) => { # [cfg (not (target_arch = "bpf"))] # [doc = " The const ID."] pub const ID : $ crate :: Address = $ crate :: Address :: from_str_const ($ address) ; # [cfg (target_arch = "bpf")] # [doc = " The const ID."] pub static ID : $ crate :: Address = $ crate :: Address :: from_str_const ($ address) ; # [doc = " Returns `true` if given address is the ID."] # [deprecated ()] pub fn check_id (id : &$ crate :: Address) -> bool { id == & ID } # [doc = " Returns the ID."] # [deprecated ()] pub const fn id () -> $ crate :: Address { # [cfg (not (target_arch = "bpf"))] { ID } # [cfg (target_arch = "bpf")] $ crate :: Address :: from_str_const ($ address) } # [cfg (test)] # [test] # [allow (deprecated)] fn test_id () { assert ! (check_id (& id ())) ; } } ; }
};
}
