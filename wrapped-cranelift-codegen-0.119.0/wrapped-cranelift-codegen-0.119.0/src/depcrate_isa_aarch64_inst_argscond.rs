// Generated macro for Cond (enum)
macro_rules! Depcrate_isa_aarch64_inst_argsCond {
() => {
// Module: crate::isa::aarch64::inst::args
// Provides: {"Cond"}
// Dependencies: {}
# [doc = " Condition for conditional branches."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum Cond { # [doc = " Equal."] Eq = 0 , # [doc = " Not equal."] Ne = 1 , # [doc = " Unsigned greater than or equal to."] Hs = 2 , # [doc = " Unsigned less than."] Lo = 3 , # [doc = " Minus, negative."] Mi = 4 , # [doc = " Positive or zero."] Pl = 5 , # [doc = " Signed overflow."] Vs = 6 , # [doc = " No signed overflow."] Vc = 7 , # [doc = " Unsigned greater than."] Hi = 8 , # [doc = " Unsigned less than or equal to."] Ls = 9 , # [doc = " Signed greater or equal to."] Ge = 10 , # [doc = " Signed less than."] Lt = 11 , # [doc = " Signed greater than."] Gt = 12 , # [doc = " Signed less than or equal."] Le = 13 , # [doc = " Always executed."] Al = 14 , # [doc = " Always executed."] Nv = 15 , }
};
}
