// Generated macro for combine_conditions (function)
macro_rules! Depcratecombine_conditions {
() => {
// Module: crate
// Provides: {"combine_conditions"}
// Dependencies: {}
# [doc = " Combine multiple conditions into a single condition expression"] fn combine_conditions (conditions : Vec < proc_macro2 :: TokenStream >) -> proc_macro2 :: TokenStream { if conditions . is_empty () { quote ! (true) } else if conditions . len () == 1 { conditions [0] . clone () } else { quote ! (# (# conditions) &&*) } }
};
}
