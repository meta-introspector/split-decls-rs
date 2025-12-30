// Generated macro for tests (module)
macro_rules! Depcrate_format_testtests {
() => {
// Module: crate::format::test
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn suite_started () { let input = r#"{ "type": "suite", "event": "started", "test_count": 10 }"# ; let _data : Event = serde_json :: from_str (input) . unwrap () ; } # [test] fn suite_ok () { let input = "{ \"type\": \"suite\", \
                     \"event\": \"ok\", \
                     \"passed\": 6, \
                     \"failed\": 5, \
                     \"allowed_fail\": 4, \
                     \"ignored\": 3, \
                     \"measured\": 2, \
                     \"filtered_out\": 1 }" ; let _data : Event = serde_json :: from_str (input) . unwrap () ; } # [test] fn suite_failed () { let input = "{ \"type\": \"suite\", \
                     \"event\": \"failed\", \
                     \"passed\": 6, \
                     \"failed\": 5, \
                     \"allowed_fail\": 4, \
                     \"ignored\": 3, \
                     \"measured\": 2, \
                     \"filtered_out\": 1 }" ; let _data : Event = serde_json :: from_str (input) . unwrap () ; } # [test] fn test_started () { let input = r#"{ "type": "test", "event": "started", "name": "foo" }"# ; let _data : Event = serde_json :: from_str (input) . unwrap () ; } # [test] fn test_timeout () { let input = r#"{ "type": "test", "event": "timeout", "name": "foo" }"# ; let _data : Event = serde_json :: from_str (input) . unwrap () ; } # [test] fn bench () { let input = "{ \"type\": \"bench\", \
                     \"name\": \"foo\", \
                     \"median\": 10, \
                     \"deviation\": 2 }" ; let _data : Event = serde_json :: from_str (input) . unwrap () ; } # [test] fn bench_full () { let input = "{ \"type\": \"bench\", \
                     \"name\": \"foo\", \
                     \"median\": 10, \
                     \"deviation\": 2, \
                     \"mib_per_second\": 1 }" ; let _data : Event = serde_json :: from_str (input) . unwrap () ; } }
};
}
