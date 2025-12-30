// Generated macro for impl_27 (impl)
macro_rules! Depcrateimpl_27 {
() => {
// Module: crate
// Provides: {"impl_27"}
// Dependencies: {}
impl < T : FloatCore > PartialEq for OrderedFloat < T > { # [inline] fn eq (& self , other : & OrderedFloat < T >) -> bool { if self . 0 . is_nan () { other . 0 . is_nan () } else { self . 0 == other . 0 } } }
};
}
