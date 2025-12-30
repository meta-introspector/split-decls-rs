// Generated macro for impl_133 (impl)
macro_rules! Depcrate_labelimpl_133 {
() => {
// Module: crate::label
// Provides: {"impl_133"}
// Dependencies: {}
impl Label { pub fn new (label : String) -> Label { always ! (label . starts_with (char :: is_uppercase) && ! label . ends_with ('.')) ; Label (label) } }
};
}
