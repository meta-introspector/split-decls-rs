// Generated macro for classes_are_sorted_and_nonoverlapping (function)
macro_rules! Depcrate_propertiesclasses_are_sorted_and_nonoverlapping {
() => {
// Module: crate::properties
// Provides: {"classes_are_sorted_and_nonoverlapping"}
// Dependencies: {}
# [test] fn classes_are_sorted_and_nonoverlapping () { fn prop (ranges : Vec < (char , char) >) -> bool { class (& ranges) . canonicalize () . windows (2) . all (| w | w [0] . end < dec_char (w [1] . start)) } qc (prop as fn (Vec < (char , char) >) -> bool) ; }
};
}
