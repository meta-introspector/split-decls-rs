// Generated macro for valid_class_ranges (function)
macro_rules! Depcrate_propertiesvalid_class_ranges {
() => {
// Module: crate::properties
// Provides: {"valid_class_ranges"}
// Dependencies: {}
# [test] fn valid_class_ranges () { fn prop (ranges : Vec < (char , char) >) -> bool { class (& ranges) . canonicalize () . into_iter () . all (| r | r . start <= r . end) } qc (prop as fn (Vec < (char , char) >) -> bool) ; }
};
}
