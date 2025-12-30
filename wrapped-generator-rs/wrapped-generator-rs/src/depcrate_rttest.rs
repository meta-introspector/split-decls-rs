// Generated macro for test (module)
macro_rules! Depcrate_rttest {
() => {
// Module: crate::rt
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: is_generator ; # [test] fn test_is_context () { assert ! (! is_generator ()) ; } # [test] fn test_overflow () { use crate :: * ; use std :: panic :: catch_unwind ; for _ in 0 .. 2 { let result = catch_unwind (| | { let mut g = Gn :: new_scoped (move | _s : Scope < () , () > | { let guard = super :: guard :: current () ; std :: hint :: black_box (unsafe { * (guard . start as * const usize) }) ; eprintln ! ("entered unreachable code") ; std :: process :: abort () ; }) ; g . next () ; }) ; assert ! (matches ! (result . map_err (| err | * err . downcast ::< Error > () . unwrap ()) , Err (Error :: StackErr))) ; } } }
};
}
