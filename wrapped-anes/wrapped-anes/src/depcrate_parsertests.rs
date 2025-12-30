// Generated macro for tests (module)
macro_rules! Depcrate_parsertests {
() => {
// Module: crate::parser
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: Parser ; # [test] fn dispatch_char () { let mut parser = Parser :: default () ; parser . advance (b"a" , false) ; assert ! (parser . next () . is_some ()) ; } # [test] fn dispatch_esc_sequence () { let mut parser = Parser :: default () ; parser . advance (b"\x1B" , true) ; assert ! (parser . next () . is_none ()) ; parser . advance (b"a" , false) ; assert ! (parser . next () . is_some ()) ; } # [test] fn does_not_dispatch_esc_sequence_with_upper_case_o () { let mut parser = Parser :: default () ; parser . advance (b"\x1B" , true) ; assert ! (parser . next () . is_none ()) ; parser . advance (b"O" , true) ; assert ! (parser . next () . is_none ()) ; } # [test] fn dispatch_esc_with_upper_case_o_followed_by_char_as_single_sequence () { let mut parser = Parser :: default () ; parser . advance (b"\x1B" , true) ; assert ! (parser . next () . is_none ()) ; parser . advance (b"O" , true) ; assert ! (parser . next () . is_none ()) ; parser . advance (b"P" , false) ; assert ! (parser . next () . is_some ()) ; assert ! (parser . next () . is_none ()) ; } # [test] fn dispatch_csi_sequence () { let mut parser = Parser :: default () ; parser . advance (b"\x1B" , true) ; assert ! (parser . next () . is_none ()) ; parser . advance (b"[" , true) ; assert ! (parser . next () . is_none ()) ; parser . advance (b"D" , false) ; assert ! (parser . next () . is_some ()) ; } }
};
}
