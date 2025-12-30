// Generated macro for process (function)
macro_rules! Depcrate_eventprocess {
() => {
// Module: crate::event
// Provides: {"process"}
// Dependencies: {}
# [doc = " Generate the syntax tree with the control of events."] pub (super) fn process (mut events : Vec < Event >) -> Output { let mut res = Output :: default () ; let mut forward_parents = Vec :: new () ; for i in 0 .. events . len () { match mem :: replace (& mut events [i] , Event :: tombstone ()) { Event :: Start { kind , forward_parent } => { forward_parents . push (kind) ; let mut idx = i ; let mut fp = forward_parent ; while let Some (fwd) = fp { idx += fwd as usize ; fp = match mem :: replace (& mut events [idx] , Event :: tombstone ()) { Event :: Start { kind , forward_parent } => { forward_parents . push (kind) ; forward_parent } _ => unreachable ! () , } ; } for kind in forward_parents . drain (..) . rev () { if kind != TOMBSTONE { res . enter_node (kind) ; } } } Event :: Finish => res . leave_node () , Event :: Token { kind , n_raw_tokens } => { res . token (kind , n_raw_tokens) ; } Event :: FloatSplitHack { ends_in_dot } => { res . float_split_hack (ends_in_dot) ; let ev = mem :: replace (& mut events [i + 1] , Event :: tombstone ()) ; assert ! (matches ! (ev , Event :: Finish) , "{ev:?}") ; } Event :: Error { msg } => res . error (msg) , } } res }
};
}
