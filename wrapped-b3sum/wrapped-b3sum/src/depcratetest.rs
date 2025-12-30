// Generated macro for test (module)
macro_rules! Depcratetest {
() => {
// Module: crate
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use clap :: CommandFactory ; # [test] fn test_args () { crate :: Inner :: command () . debug_assert () ; } }
};
}
