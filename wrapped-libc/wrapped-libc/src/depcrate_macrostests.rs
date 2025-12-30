// Generated macro for tests (module)
macro_rules! Depcrate_macrostests {
() => {
// Module: crate::macros
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { # [test] fn c_enumbasic () { c_enum ! { pub enum e { VAR0 , VAR1 , VAR2 , } } assert_eq ! (VAR0 , 0_u32) ; assert_eq ! (VAR1 , 1_u32) ; assert_eq ! (VAR2 , 2_u32) ; } # [test] fn c_enumrepr () { c_enum ! { # [repr (u16)] pub enum e { VAR0 , } } assert_eq ! (VAR0 , 0_u16) ; } # [test] fn c_enumset_value () { c_enum ! { pub enum e { VAR2 = 2 , VAR3 , VAR4 , } } assert_eq ! (VAR2 , 2_u32) ; assert_eq ! (VAR3 , 3_u32) ; assert_eq ! (VAR4 , 4_u32) ; } # [test] fn c_enummultiple_set_value () { c_enum ! { pub enum e { VAR0 , VAR2_0 = 2 , VAR3_0 , VAR4_0 , VAR2_1 = 2 , VAR3_1 , VAR4_1 , } } assert_eq ! (VAR0 , 0_u32) ; assert_eq ! (VAR2_0 , 2_u32) ; assert_eq ! (VAR3_0 , 3_u32) ; assert_eq ! (VAR4_0 , 4_u32) ; assert_eq ! (VAR2_1 , 2_u32) ; assert_eq ! (VAR3_1 , 3_u32) ; assert_eq ! (VAR4_1 , 4_u32) ; } }
};
}
