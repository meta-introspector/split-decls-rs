// Generated macro for test (module)
macro_rules! Depcrate_output_render_gittest {
() => {
// Module: crate::output::render::git
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] pub mod test { use super :: Colours ; use crate :: fs :: fields as f ; use crate :: output :: cell :: { DisplayWidth , TextCell } ; use nu_ansi_term :: Color :: * ; use nu_ansi_term :: Style ; struct TestColours ; impl Colours for TestColours { fn not_modified (& self) -> Style { Fixed (90) . normal () } fn new (& self) -> Style { Fixed (91) . normal () } fn modified (& self) -> Style { Fixed (92) . normal () } fn deleted (& self) -> Style { Fixed (93) . normal () } fn renamed (& self) -> Style { Fixed (94) . normal () } fn type_change (& self) -> Style { Fixed (95) . normal () } fn ignored (& self) -> Style { Fixed (96) . normal () } fn conflicted (& self) -> Style { Fixed (97) . normal () } } # [test] fn git_blank () { let stati = f :: Git { staged : f :: GitStatus :: NotModified , unstaged : f :: GitStatus :: NotModified , } ; let expected = TextCell { width : DisplayWidth :: from (2) , contents : vec ! [Fixed (90) . paint ("-") , Fixed (90) . paint ("-")] . into () , } ; assert_eq ! (expected , stati . render (& TestColours)) ; } # [test] fn git_new_changed () { let stati = f :: Git { staged : f :: GitStatus :: New , unstaged : f :: GitStatus :: Modified , } ; let expected = TextCell { width : DisplayWidth :: from (2) , contents : vec ! [Fixed (91) . paint ("N") , Fixed (92) . paint ("M")] . into () , } ; assert_eq ! (expected , stati . render (& TestColours)) ; } }
};
}
