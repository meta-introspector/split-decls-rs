// Generated macro for timeout_tests (module)
macro_rules! Depcrate_test_runner_runnertimeout_tests {
() => {
// Module: crate::test_runner::runner
// Provides: {"timeout_tests"}
// Dependencies: {}
# [cfg (all (feature = "fork" , feature = "timeout" , test))] mod timeout_tests { use core :: u32 ; use std :: thread ; use std :: time :: Duration ; use super :: * ; rusty_fork_test ! { #! [rusty_fork (timeout_ms = 4_000)] # [test] fn max_shrink_iters_works () { test_shrink_bail (Config { max_shrink_iters : 5 , .. Config :: default () }) ; } # [test] fn max_shrink_time_works () { test_shrink_bail (Config { max_shrink_time : 1000 , .. Config :: default () }) ; } # [test] fn max_shrink_iters_works_with_forking () { test_shrink_bail (Config { fork : true , test_name : Some (concat ! (module_path ! () , "::max_shrink_iters_works_with_forking")) , max_shrink_time : 1000 , .. Config :: default () }) ; } # [test] fn detects_child_failure_to_start () { let mut runner = TestRunner :: new (Config { timeout : 100 , test_name : Some (concat ! (module_path ! () , "::detects_child_failure_to_start")) , .. Config :: default () }) ; let result = runner . run (& Just (()) . prop_map (| () | { thread :: sleep (Duration :: from_millis (200)) }) , Ok) ; if let Err (TestError :: Abort (_)) = result { } else { panic ! ("Unexpected result: {:?}" , result) ; } } } fn test_shrink_bail (config : Config) { let mut runner = TestRunner :: new (config) ; let result = runner . run (& crate :: num :: u64 :: ANY , | v | { thread :: sleep (Duration :: from_millis (250)) ; prop_assert ! (v <= u32 :: MAX as u64) ; Ok (()) }) ; if let Err (TestError :: Fail (_ , value)) = result { assert ! (value > u32 :: MAX as u64) ; } else { panic ! ("Unexpected result: {:?}" , result) ; } } }
};
}
