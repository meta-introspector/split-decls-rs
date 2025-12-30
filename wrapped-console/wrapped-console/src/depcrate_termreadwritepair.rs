// Generated macro for ReadWritePair (struct)
macro_rules! Depcrate_termReadWritePair {
() => {
// Module: crate::term
// Provides: {"ReadWritePair"}
// Dependencies: {}
# [cfg (unix)] # [derive (Debug , Clone)] pub struct ReadWritePair { # [allow (unused)] read : Arc < Mutex < dyn TermRead > > , write : Arc < Mutex < dyn TermWrite > > , style : Style , }
};
}
