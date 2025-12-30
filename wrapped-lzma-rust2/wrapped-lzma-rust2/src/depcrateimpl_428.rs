// Generated macro for impl_428 (impl)
macro_rules! Depcrateimpl_428 {
() => {
// Module: crate
// Provides: {"impl_428"}
// Dependencies: {}
impl < T : AutoFinish > Drop for AutoFinisher < T > { fn drop (& mut self) { if let Some (writer) = self . 0 . take () { writer . finish_ignore_error () ; } } }
};
}
