// Generated macro for tests (module)
macro_rules! Depcrate_windowstests {
() => {
// Module: crate::windows
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: super :: { IterationControl , Segment , SharedLibrary } ; use crate :: windows ; # [test] fn can_break () { let mut first_count = 0 ; windows :: SharedLibrary :: each (| _ | { first_count += 1 ; }) ; assert ! (first_count > 2) ; let mut second_count = 0 ; windows :: SharedLibrary :: each (| _ | { second_count += 1 ; if second_count == first_count - 1 { IterationControl :: Break } else { IterationControl :: Continue } }) ; assert_eq ! (second_count , first_count - 1) ; } # [test] fn get_name () { windows :: SharedLibrary :: each (| shlib | { let _ = shlib . name () ; assert ! (shlib . debug_name () . is_some ()) ; }) ; } # [test] fn have_code () { windows :: SharedLibrary :: each (| shlib | { println ! ("shlib = {:?}" , shlib . name ()) ; let mut found_code = false ; for seg in shlib . segments () { println ! ("    segment = {:?}" , seg . name ()) ; if seg . is_code () { found_code = true ; } } assert ! (found_code) ; }) ; } # [test] fn get_id () { windows :: SharedLibrary :: each (| shlib | { assert ! (shlib . id () . is_some ()) ; assert ! (shlib . debug_id () . is_some ()) ; }) ; } }
};
}
