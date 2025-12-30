// Generated macro for compare_labels (function)
macro_rules! Depcrate_needless_continuecompare_labels {
() => {
// Module: crate::needless_continue
// Provides: {"compare_labels"}
// Dependencies: {}
# [doc = " If the `continue` has a label, check it matches the label of the loop."] fn compare_labels (loop_label : Option < & Label > , continue_label : Option < & Label >) -> bool { match (loop_label , continue_label) { (_ , None) => true , (None , _) => false , (Some (x) , Some (y)) => x . ident == y . ident , } }
};
}
