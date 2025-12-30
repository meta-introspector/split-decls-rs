// Generated macro for CheckCtx (struct)
macro_rules! Depcrate_run_cfgCheckCtx {
() => {
// Module: crate::run_cfg
// Provides: {"CheckCtx"}
// Dependencies: {}
# [doc = " Context passed to [`CheckOutput`]."] # [derive (Clone , Debug , PartialEq , Eq)] pub struct CheckCtx { # [doc = " Allowed ULP deviation"] pub ulp : u32 , pub fn_ident : Identifier , pub base_name : BaseName , # [doc = " Function name."] pub fn_name : & 'static str , # [doc = " Return the unsuffixed version of the function name."] pub base_name_str : & 'static str , # [doc = " Source of truth for tests."] pub basis : CheckBasis , pub gen_kind : GeneratorKind , pub extensive : bool , # [doc = " If specified, this value will override the value returned by [`iteration_count`]."] pub override_iterations : Option < u64 > , }
};
}
