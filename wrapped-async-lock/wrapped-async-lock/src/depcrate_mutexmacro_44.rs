// Generated macro for macro_44 (macro)
macro_rules! Depcrate_mutexmacro_44 {
() => {
// Module: crate::mutex
// Provides: {"macro_44"}
// Dependencies: {}
pin_project_lite :: pin_project ! { # [doc = " Inner future for acquiring the mutex."] struct LockInner <'a , T : ? Sized > { mutex : &'a Mutex < T >, # [pin] acquire_slow : Option < AcquireSlow <&'a Mutex < T >, T >>, } }
};
}
