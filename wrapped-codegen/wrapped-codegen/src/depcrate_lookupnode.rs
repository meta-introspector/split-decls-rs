// Generated macro for node (function)
macro_rules! Depcrate_lookupnode {
() => {
// Module: crate::lookup
// Provides: {"node"}
// Dependencies: {}
pub fn node < 'a > (defs : & 'a Definitions , name : & str) -> & 'a Node { for node in & defs . types { if node . ident == name { return node ; } } panic ! ("not found: {}" , name) }
};
}
