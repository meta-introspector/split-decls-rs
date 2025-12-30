// Generated macro for lock_unlock (function)
macro_rules! Depcrate_tests_locklock_unlock {
() => {
// Module: crate::tests::lock
// Provides: {"lock_unlock"}
// Dependencies: {}
# [test] fn lock_unlock () { let lock = NSLock :: new () ; unsafe { lock . lock () ; assert ! (! lock . tryLock ()) ; lock . unlock () ; assert ! (lock . tryLock ()) ; lock . unlock () ; } }
};
}
