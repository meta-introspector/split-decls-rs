// Generated macro for Guard (struct)
macro_rules! Depcrate_rawGuard {
() => {
// Module: crate::raw
// Provides: {"Guard"}
// Dependencies: {}
# [doc = " A guard that closes the task if polling its future panics."] struct Guard < F , T > (* const () , & 'static TaskLayout , PhantomData < fn () -> F >) where F : Future < Output = T > ;
};
}
