// Generated macro for tests (module)
macro_rules! Depcrate_ir_condcodestests {
() => {
// Module: crate::ir::condcodes
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use std :: string :: ToString ; # [test] fn int_complement () { for r in IntCC :: all () { let cc = * r ; let inv = cc . complement () ; assert ! (cc != inv) ; assert_eq ! (inv . complement () , cc) ; } } # [test] fn int_swap_args () { for r in IntCC :: all () { let cc = * r ; let rev = cc . swap_args () ; assert_eq ! (rev . swap_args () , cc) ; } } # [test] fn int_display () { for r in IntCC :: all () { let cc = * r ; assert_eq ! (cc . to_string () . parse () , Ok (cc)) ; } assert_eq ! ("bogus" . parse ::< IntCC > () , Err (())) ; } # [test] fn float_complement () { for r in FloatCC :: all () { let cc = * r ; let inv = cc . complement () ; assert ! (cc != inv) ; assert_eq ! (inv . complement () , cc) ; } } # [test] fn float_swap_args () { for r in FloatCC :: all () { let cc = * r ; let rev = cc . swap_args () ; assert_eq ! (rev . swap_args () , cc) ; } } # [test] fn float_display () { for r in FloatCC :: all () { let cc = * r ; assert_eq ! (cc . to_string () . parse () , Ok (cc)) ; } assert_eq ! ("bogus" . parse ::< FloatCC > () , Err (())) ; } }
};
}
