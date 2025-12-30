// Generated macro for Set (struct)
macro_rules! Depcrate_setSet {
() => {
// Module: crate::set
// Provides: {"Set"}
// Dependencies: {}
# [doc = " An immutable set constructed at compile time."] # [doc = ""] # [doc = " ## Note"] # [doc = ""] # [doc = " The fields of this struct are public so that they may be initialized by the"] # [doc = " `phf_set!` macro and code generation. They are subject to change at any"] # [doc = " time and should never be accessed directly."] pub struct Set < T : 'static > { # [doc (hidden)] pub map : Map < T , () > , }
};
}
