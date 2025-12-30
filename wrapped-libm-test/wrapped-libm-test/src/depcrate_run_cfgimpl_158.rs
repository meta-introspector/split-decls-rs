// Generated macro for impl_158 (impl)
macro_rules! Depcrate_run_cfgimpl_158 {
() => {
// Module: crate::run_cfg
// Provides: {"impl_158"}
// Dependencies: {}
impl CheckCtx { # [doc = " Create a new check context, using the default ULP for the function."] pub fn new (fn_ident : Identifier , basis : CheckBasis , gen_kind : GeneratorKind) -> Self { let mut ret = Self { ulp : 0 , fn_ident , fn_name : fn_ident . as_str () , base_name : fn_ident . base_name () , base_name_str : fn_ident . base_name () . as_str () , basis , gen_kind , extensive : false , override_iterations : None , } ; ret . ulp = crate :: default_ulp (& ret) ; ret } # [doc = " Configure that this is an extensive test."] pub fn extensive (mut self , extensive : bool) -> Self { self . extensive = extensive ; self } # [doc = " The number of input arguments for this function."] pub fn input_count (& self) -> usize { self . fn_ident . math_op () . rust_sig . args . len () } pub fn override_iterations (& mut self , count : u64) { self . override_iterations = Some (count) } }
};
}
