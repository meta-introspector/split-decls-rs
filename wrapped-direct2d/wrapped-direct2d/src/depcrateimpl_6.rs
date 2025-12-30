// Generated macro for impl_6 (impl)
macro_rules! Depcrateimpl_6 {
() => {
// Module: crate
// Provides: {"impl_6"}
// Dependencies: {}
impl Angles { fn now () -> Self { let time = unsafe { GetLocalTime () } ; let second = (time . wSecond as f32 + time . wMilliseconds as f32 / 1000.0) * 6.0 ; let minute = time . wMinute as f32 * 6.0 + second / 60.0 ; let hour = (time . wHour % 12) as f32 * 30.0 + minute / 12.0 ; Self { second , minute , hour , } } }
};
}
