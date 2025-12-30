// Generated macro for tests (module)
macro_rules! Depcrate_ir_trapcodetests {
() => {
// Module: crate::ir::trapcode
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use alloc :: string :: ToString ; # [test] fn display () { for r in TrapCode :: non_user_traps () { let tc = * r ; assert_eq ! (tc . to_string () . parse () , Ok (tc)) ; } assert_eq ! ("bogus" . parse ::< TrapCode > () , Err (())) ; assert_eq ! (TrapCode :: unwrap_user (17) . to_string () , "user17") ; assert_eq ! ("user22" . parse () , Ok (TrapCode :: unwrap_user (22))) ; assert_eq ! ("user" . parse ::< TrapCode > () , Err (())) ; assert_eq ! ("user-1" . parse ::< TrapCode > () , Err (())) ; assert_eq ! ("users" . parse ::< TrapCode > () , Err (())) ; } }
};
}
