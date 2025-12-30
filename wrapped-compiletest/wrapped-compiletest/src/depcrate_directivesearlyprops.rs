// Generated macro for EarlyProps (struct)
macro_rules! Depcrate_directivesEarlyProps {
() => {
// Module: crate::directives
// Provides: {"EarlyProps"}
// Dependencies: {}
# [doc = " Properties which must be known very early, before actually running"] # [doc = " the test."] # [derive (Default)] pub struct EarlyProps { # [doc = " Auxiliary crates that should be built and made available to this test."] # [doc = " Included in [`EarlyProps`] so that the indicated files can participate"] # [doc = " in up-to-date checking. Building happens via [`TestProps::aux`] instead."] pub (crate) aux : AuxProps , pub revisions : Vec < String > , }
};
}
