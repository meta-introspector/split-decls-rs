// Generated macro for end (macro)
macro_rules! Depcrate_serend {
() => {
// Module: crate::ser
// Provides: {"end"}
// Dependencies: {}
macro_rules ! end { () => { # [inline] fn end (self) -> Result < () , Self :: Error > { if self . ending { self . encoder . 0 . push (Header :: Break) ?; } Ok (()) } } ; }
};
}
