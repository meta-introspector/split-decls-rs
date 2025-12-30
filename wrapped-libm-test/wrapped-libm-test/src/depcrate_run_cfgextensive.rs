// Generated macro for EXTENSIVE (static)
macro_rules! Depcrate_run_cfgEXTENSIVE {
() => {
// Module: crate::run_cfg
// Provides: {"EXTENSIVE"}
// Dependencies: {}
# [doc = " A list of all functions that should get extensive tests, as configured by environment variable."] # [doc = ""] # [doc = " This also supports the special test name `all` to run all tests, as well as `all_f16`,"] # [doc = " `all_f32`, `all_f64`, and `all_f128` to run all tests for a specific float type."] static EXTENSIVE : LazyLock < Vec < Identifier > > = LazyLock :: new (| | { let var = env :: var (EXTENSIVE_ENV) . unwrap_or_default () ; let list = var . split (",") . filter (| s | ! s . is_empty ()) . collect :: < Vec < _ > > () ; let mut ret = Vec :: new () ; let append_ty_ops = | ret : & mut Vec < _ > , fty : FloatTy | { let iter = Identifier :: ALL . iter () . filter (move | id | id . math_op () . float_ty == fty) . copied () ; ret . extend (iter) ; } ; for item in list { match item { "all" => ret = Identifier :: ALL . to_owned () , "all_f16" => append_ty_ops (& mut ret , FloatTy :: F16) , "all_f32" => append_ty_ops (& mut ret , FloatTy :: F32) , "all_f64" => append_ty_ops (& mut ret , FloatTy :: F64) , "all_f128" => append_ty_ops (& mut ret , FloatTy :: F128) , s => { let id = Identifier :: from_str (s) . unwrap_or_else (| | panic ! ("unrecognized test name `{s}`")) ; ret . push (id) ; } } } ret }) ;
};
}
