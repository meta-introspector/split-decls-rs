// Generated macro for SREG (static)
macro_rules! Depcrate_registrySREG {
() => {
// Module: crate::registry
// Provides: {"SREG"}
// Dependencies: {}
static SREG : Lazy < Mutex < HashMap < usize , SystemRegistry > > > = Lazy :: new (| | Mutex :: new (HashMap :: new ())) ;
};
}
