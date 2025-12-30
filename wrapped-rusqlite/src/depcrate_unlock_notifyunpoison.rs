// Generated macro for unpoison (function)
macro_rules! Depcrate_unlock_notifyunpoison {
() => {
// Module: crate::unlock_notify
// Provides: {"unpoison"}
// Dependencies: {}
# [inline] fn unpoison < T > (r : Result < T , std :: sync :: PoisonError < T > >) -> T { r . unwrap_or_else (std :: sync :: PoisonError :: into_inner) }
};
}
