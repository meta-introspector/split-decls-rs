// Generated macro for macro_67 (macro)
macro_rules! Depcratemacro_67 {
() => {
// Module: crate
// Provides: {"macro_67"}
// Dependencies: {}
pin_project ! { # [doc = " A wrapper around a future, running a closure when dropped."] struct AsyncCallOnDrop < Fut , Cleanup : FnMut () > { # [pin] future : Fut , cleanup : CallOnDrop < Cleanup >, } }
};
}
