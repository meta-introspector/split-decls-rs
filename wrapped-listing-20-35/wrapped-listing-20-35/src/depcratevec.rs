// Generated macro for vec (macro)
macro_rules! Depcratevec {
() => {
// Module: crate
// Provides: {"vec"}
// Dependencies: {}
# [macro_export] macro_rules ! vec { ($ ($ x : expr) ,*) => { { let mut temp_vec = Vec :: new () ; $ (temp_vec . push ($ x) ;) * temp_vec } } ; }
};
}
