// Generated macro for num_threads (function)
macro_rules! Depcratenum_threads {
() => {
// Module: crate
// Provides: {"num_threads"}
// Dependencies: {}
# [doc = " Obtain the number of threads currently part of the active process. Returns `None` if the number"] # [doc = " of threads cannot be determined."] pub fn num_threads () -> Option < NonZeroUsize > { imp :: num_threads () }
};
}
