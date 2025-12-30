// Generated macro for CallUnstable (struct)
macro_rules! Depcrate_check_consts_opsCallUnstable {
() => {
// Module: crate::check_consts::ops
// Provides: {"CallUnstable"}
// Dependencies: {}
# [doc = " A call to an `#[unstable]` const fn, `#[rustc_const_unstable]` function or trait."] # [doc = ""] # [doc = " Contains the name of the feature that would allow the use of this function/trait."] # [derive (Debug)] pub (crate) struct CallUnstable { pub def_id : DefId , pub feature : Symbol , # [doc = " If this is true, then the feature is enabled, but we need to still check if it is safe to"] # [doc = " expose on stable."] pub feature_enabled : bool , pub safe_to_expose_on_stable : bool , # [doc = " true if `def_id` is the function we are calling, false if `def_id` is an unstable trait."] pub is_function_call : bool , }
};
}
