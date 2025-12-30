// Generated macro for int_range (function)
macro_rules! Depcrate_run_cfgint_range {
() => {
// Module: crate::run_cfg
// Provides: {"int_range"}
// Dependencies: {}
# [doc = " Some tests require that an integer be kept within reasonable limits; generate that here."] pub fn int_range (ctx : & CheckCtx , argnum : usize) -> RangeInclusive < i32 > { let t_env = TestEnv :: from_env (ctx) ; if ! matches ! (ctx . base_name , BaseName :: Jn | BaseName :: Yn) { return i32 :: MIN ..= i32 :: MAX ; } assert_eq ! (argnum , 0 , "For `jn`/`yn`, only the first argument takes an integer") ; let non_extensive_range = if t_env . slow_platform || ! cfg ! (optimizations_enabled) { (- 0xf) ..= 0xff } else { (- 0xff) ..= 0xffff } ; let extensive_range = (- 0xfff) ..= 0xfffff ; match ctx . gen_kind { _ if ctx . extensive => extensive_range , GeneratorKind :: Spaced | GeneratorKind :: Random => non_extensive_range , GeneratorKind :: EdgeCases => extensive_range , GeneratorKind :: List => unimplemented ! ("shoudn't need range for {:?}" , ctx . gen_kind) , } }
};
}
