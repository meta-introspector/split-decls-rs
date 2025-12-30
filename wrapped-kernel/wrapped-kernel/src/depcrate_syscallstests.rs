// Generated macro for tests (module)
macro_rules! Depcrate_syscallstests {
() => {
// Module: crate::syscalls
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use core :: ptr ; use super :: * ; # [cfg (target_os = "none")] # [test_case] fn test_get_application_parameters () { crate :: env :: init () ; let (argc , argv , _envp) = get_application_parameters () ; assert_ne ! (argc , 0) ; assert_ne ! (argv , ptr :: null ()) ; } }
};
}
