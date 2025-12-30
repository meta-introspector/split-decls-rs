// Generated macro for Node (struct)
macro_rules! DepcrateNode {
() => {
// Module: crate
// Provides: {"Node"}
// Dependencies: {}
struct Node { id : Option < usize > , left : Option < Box < Node > > , right : Option < Box < Node > > , terminal : Option < usize > , maybe_eos : bool , transitions : RefCell < Vec < Transition > > , }
};
}
