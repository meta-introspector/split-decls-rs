// Generated macro for length_option_helper (macro)
macro_rules! Depcrate_fieldsetslength_option_helper {
() => {
// Module: crate::fieldsets
// Provides: {"length_option_helper"}
// Dependencies: {}
# [doc = " Generates the options argument passed into the docs test constructor"] macro_rules ! length_option_helper { ($ type : ty , $ length : ident) => { concat ! (stringify ! ($ type) , "::" , stringify ! ($ length) , "()") } ; }
};
}
