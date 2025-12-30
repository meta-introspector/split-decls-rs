// Generated macro for tests (module)
macro_rules! Depcrate_check_macrostests {
() => {
// Module: crate::check_macros
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { # [allow (unused_variables)] # [allow (unused_must_use)] # [test] # [ignore = "Testing in compile time"] fn test_check () { let mut session = crate :: spawn ("") . unwrap () ; crate :: check ! { & mut session , as11d = "zxc" => { } , } ; crate :: check ! { & mut session , as11d = "zxc" => { } , asbb = "zxc123" => { } , } ; crate :: check ! { session , } ; crate :: check ! { & mut session , as11d = "zxc" => { } asbb = "zxc123" => { } } ; crate :: check ! { & mut session , as11d = "zxc" => { } default => { } } ; # [cfg (not (feature = "async"))] { crate :: check ! { & mut session , as11d = "zxc" => { } , } . unwrap () ; (crate :: check ! { & mut session , as11d = "zxc" => { } , }) . unwrap () ; (crate :: check ! { & mut session , as11d = "zxc" => { } , }) . unwrap () ; (crate :: check ! { & mut session , as11d = "zxc" => { println ! ("asd") } , }) . unwrap () ; } # [cfg (feature = "async")] async { crate :: check ! { & mut session , as11d = "zxc" => { } , } . await . unwrap () ; (crate :: check ! { & mut session , as11d = "zxc" => { } , }) . await . unwrap () ; (crate :: check ! { & mut session , as11d = "zxc" => { } , }) . await . unwrap () ; (crate :: check ! { & mut session , as11d = "zxc" => { println ! ("asd") } , }) . await . unwrap () ; } ; } }
};
}
