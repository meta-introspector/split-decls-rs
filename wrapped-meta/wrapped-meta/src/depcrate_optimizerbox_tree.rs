// Generated macro for box_tree (macro)
macro_rules! Depcrate_optimizerbox_tree {
() => {
// Module: crate::optimizer
// Provides: {"box_tree"}
// Dependencies: {}
# [cfg (test)] macro_rules ! box_tree { ($ node : ident ($ ($ child : ident ($ ($ args : tt) *)) ,+)) => ($ node ($ (Box :: new (box_tree ! ($ child ($ ($ args) *)))) ,+)) ; ($ expr : expr) => ($ expr) ; }
};
}
