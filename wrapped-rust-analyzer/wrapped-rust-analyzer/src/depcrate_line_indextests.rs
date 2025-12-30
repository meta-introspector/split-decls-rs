// Generated macro for tests (module)
macro_rules! Depcrate_line_indextests {
() => {
// Module: crate::line_index
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn unix () { let src = "a\nb\nc\n\n\n\n" ; let (res , endings) = LineEndings :: normalize (src . into ()) ; assert_eq ! (endings , LineEndings :: Unix) ; assert_eq ! (res , src) ; } # [test] fn dos () { let src = "\r\na\r\n\r\nb\r\nc\r\n\r\n\r\n\r\n" ; let (res , endings) = LineEndings :: normalize (src . into ()) ; assert_eq ! (endings , LineEndings :: Dos) ; assert_eq ! (res , "\na\n\nb\nc\n\n\n\n") ; } # [test] fn mixed () { let src = "a\r\nb\r\nc\r\n\n\r\n\n" ; let (res , endings) = LineEndings :: normalize (src . into ()) ; assert_eq ! (endings , LineEndings :: Dos) ; assert_eq ! (res , "a\nb\nc\n\n\n\n") ; } # [test] fn none () { let src = "abc" ; let (res , endings) = LineEndings :: normalize (src . into ()) ; assert_eq ! (endings , LineEndings :: Unix) ; assert_eq ! (res , src) ; } }
};
}
