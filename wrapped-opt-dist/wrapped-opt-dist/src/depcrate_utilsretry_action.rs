// Generated macro for retry_action (function)
macro_rules! Depcrate_utilsretry_action {
() => {
// Module: crate::utils
// Provides: {"retry_action"}
// Dependencies: {}
# [allow (unused)] pub fn retry_action < F : Fn () -> anyhow :: Result < R > , R > (action : F , name : & str , count : u64 ,) -> anyhow :: Result < R > { for attempt in 0 .. count { match action () { Ok (result) => return Ok (result) , Err (error) => { log :: error ! ("Failed to perform action `{name}`, attempt #{attempt}: {error:?}") ; std :: thread :: sleep (Duration :: from_secs (5)) ; } } } Err (anyhow :: anyhow ! ("Failed to perform action `{name}` after {count} retries")) }
};
}
