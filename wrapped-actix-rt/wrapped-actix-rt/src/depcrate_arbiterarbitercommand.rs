// Generated macro for ArbiterCommand (enum)
macro_rules! Depcrate_arbiterArbiterCommand {
() => {
// Module: crate::arbiter
// Provides: {"ArbiterCommand"}
// Dependencies: {}
pub (crate) enum ArbiterCommand { Stop , Execute (Pin < Box < dyn Future < Output = () > + Send > >) , }
};
}
