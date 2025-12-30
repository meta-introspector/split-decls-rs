// Generated macro for impl_534 (impl)
macro_rules! Depcrate_nfa_thompson_builderimpl_534 {
() => {
// Module: crate::nfa::thompson::builder
// Provides: {"impl_534"}
// Dependencies: {}
impl State { # [doc = " If this state is an unconditional epsilon transition, then this returns"] # [doc = " the target of the transition."] fn goto (& self) -> Option < StateID > { match * self { State :: Empty { next } => Some (next) , State :: Union { ref alternates } if alternates . len () == 1 => { Some (alternates [0]) } State :: UnionReverse { ref alternates } if alternates . len () == 1 => { Some (alternates [0]) } _ => None , } } # [doc = " Returns the heap memory usage, in bytes, of this state."] fn memory_usage (& self) -> usize { match * self { State :: Empty { .. } | State :: ByteRange { .. } | State :: Look { .. } | State :: CaptureStart { .. } | State :: CaptureEnd { .. } | State :: Fail | State :: Match { .. } => 0 , State :: Sparse { ref transitions } => { transitions . len () * mem :: size_of :: < Transition > () } State :: Union { ref alternates } => { alternates . len () * mem :: size_of :: < StateID > () } State :: UnionReverse { ref alternates } => { alternates . len () * mem :: size_of :: < StateID > () } } } }
};
}
