// Generated macro for tests (module)
macro_rules! Depcrate_displaytests {
() => {
// Module: crate::display
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { pub use super :: super :: ANSIStrings ; pub use crate :: style :: Style ; pub use crate :: style :: Colour :: * ; # [test] fn no_control_codes_for_plain () { let one = Style :: default () . paint ("one") ; let two = Style :: default () . paint ("two") ; let output = format ! ("{}" , ANSIStrings (& [one , two])) ; assert_eq ! (&* output , "onetwo") ; } }
};
}
