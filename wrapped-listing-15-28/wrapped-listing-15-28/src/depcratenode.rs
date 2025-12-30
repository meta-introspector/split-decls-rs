// Generated macro for Node (struct)
macro_rules! DepcrateNode {
() => {
// Module: crate
// Provides: {"Node"}
// Dependencies: {}
# [derive (Debug)] struct Node { value : i32 , parent : RefCell < Weak < Node > > , children : RefCell < Vec < Rc < Node > > > , }
};
}
