// Generated macro for rt (function)
macro_rules! Depcratert {
() => {
// Module: crate
// Provides: {"rt"}
// Dependencies: {}
pub fn rt () -> Runtime { Builder :: new_current_thread () . enable_all () . build () . unwrap () }
};
}
