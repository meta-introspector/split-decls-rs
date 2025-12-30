// Generated macro for ArgAll (trait)
macro_rules! Depcrate_arg_msgargArgAll {
() => {
// Module: crate::arg::msgarg
// Provides: {"ArgAll"}
// Dependencies: {}
# [doc = " Helper trait to introspect many arguments."] pub trait ArgAll { # [doc = " A tuple of &static str. Used for introspection."] # [allow (non_camel_case_types)] type strs ; # [doc = " Enumerates all arguments with their signatures (introspection helper method)."] fn strs_sig < F : FnMut (& 'static str , Signature < 'static >) > (a : Self :: strs , f : F) ; }
};
}
