// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: uses_named_pipe_jobserver ; use super :: Version ; # [test] fn test_cmake_version () { let text = "cmake version 3.22.2

CMake suite maintained and supported by Kitware (kitware.com/cmake).
" ; let v = Version :: parse (text) . unwrap () ; assert_eq ! (v , Version :: new (3 , 22)) ; assert ! (Version :: new (3 , 22) > Version :: new (3 , 21)) ; assert ! (Version :: new (3 , 22) < Version :: new (3 , 23)) ; let _v = Version :: from_command ("cmake" . as_ref ()) . unwrap () ; } # [test] fn test_uses_fifo_jobserver () { assert ! (uses_named_pipe_jobserver ("-j --jobserver-auth=fifo:/foo" . as_ref ())) ; assert ! (! uses_named_pipe_jobserver ("-j --jobserver-auth=8:9" . as_ref ())) ; } }
};
}
