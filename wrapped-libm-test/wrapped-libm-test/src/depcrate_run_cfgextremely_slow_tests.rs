// Generated macro for EXTREMELY_SLOW_TESTS (const)
macro_rules! Depcrate_run_cfgEXTREMELY_SLOW_TESTS {
() => {
// Module: crate::run_cfg
// Provides: {"EXTREMELY_SLOW_TESTS"}
// Dependencies: {}
# [doc = " Specific tests that need to have a reduced amount of iterations to complete in a reasonable"] # [doc = " amount of time."] const EXTREMELY_SLOW_TESTS : & [SlowTest] = & [SlowTest { ident : Identifier :: Fmodf128 , gen_kind : GeneratorKind :: Spaced , extensive : false , reduce_factor : 50 , } , SlowTest { ident : Identifier :: Fmodf128 , gen_kind : GeneratorKind :: Spaced , extensive : true , reduce_factor : 50 , } ,] ;
};
}
