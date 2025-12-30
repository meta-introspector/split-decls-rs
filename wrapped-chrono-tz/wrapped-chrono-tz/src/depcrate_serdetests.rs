// Generated macro for tests (module)
macro_rules! Depcrate_serdetests {
() => {
// Module: crate::serde
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: timezones :: Tz :: { self , Etc__UTC , Europe__London , UTC } ; use serde_test :: { assert_de_tokens_error , assert_tokens , Token } ; # [test] fn serde_ok_both_ways () { assert_tokens (& Europe__London , & [Token :: String ("Europe/London")]) ; assert_tokens (& Etc__UTC , & [Token :: String ("Etc/UTC")]) ; assert_tokens (& UTC , & [Token :: String ("UTC")]) ; } # [test] fn serde_de_error () { assert_de_tokens_error :: < Tz > (& [Token :: Str ("Europe/L")] , "failed to parse timezone: 'Europe/L'" ,) ; } }
};
}
