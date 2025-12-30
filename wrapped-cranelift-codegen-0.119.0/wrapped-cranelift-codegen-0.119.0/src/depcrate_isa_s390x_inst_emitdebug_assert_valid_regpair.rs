// Generated macro for debug_assert_valid_regpair (macro)
macro_rules! Depcrate_isa_s390x_inst_emitdebug_assert_valid_regpair {
() => {
// Module: crate::isa::s390x::inst::emit
// Provides: {"debug_assert_valid_regpair"}
// Dependencies: {}
# [doc = " Debug macro for testing that a regpair is valid: that the high register is even, and the low"] # [doc = " register is one higher than the high register."] macro_rules ! debug_assert_valid_regpair { ($ hi : expr , $ lo : expr) => { if cfg ! (debug_assertions) { match ($ hi . to_real_reg () , $ lo . to_real_reg ()) { (Some (hi) , Some (lo)) => { assert ! (hi . hw_enc () % 2 == 0 , "High register is not even: {}" , show_reg ($ hi)) ; assert_eq ! (hi . hw_enc () + 1 , lo . hw_enc () , "Low register is not valid: {}, {}" , show_reg ($ hi) , show_reg ($ lo)) ; } _ => { panic ! ("Expected real registers for {} {}" , show_reg ($ hi) , show_reg ($ lo)) ; } } } } ; }
};
}
