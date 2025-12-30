// Generated macro for expect_action (function)
macro_rules! Depcrate_utilsexpect_action {
() => {
// Module: crate::utils
// Provides: {"expect_action"}
// Dependencies: {}
# [track_caller] pub fn expect_action < T > (res : Result < T , impl Display > , action : ErrAction , path : impl AsRef < Path >) -> T { match res { Ok (x) => x , Err (ref e) => panic_action (e , action , path . as_ref ()) , } }
};
}
