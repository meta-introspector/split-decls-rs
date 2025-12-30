// Generated macro for Permit (struct)
macro_rules! DepcratePermit {
() => {
// Module: crate
// Provides: {"Permit"}
// Dependencies: {}
# [doc = " A struct for cancelling operations."] # [doc = ""] # [doc = " Use [`new_sub()`](#method.new_sub) to make a subordinate permit."] # [doc = " Call [`revoke()`](#method.revoke) to revoke a permit"] # [doc = " and its subordinate permits, recursively."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " Graceful shutdown:"] # [doc = " ```"] # [doc = " # fn wait_for_shutdown_signal() { () }"] # [doc = " let top_permit = permit::Permit::new();"] # [doc = " // Start some worker threads."] # [doc = " for _ in 0..5 {"] # [doc = "     let permit = top_permit.new_sub();"] # [doc = "     std::thread::spawn(move || {"] # [doc = "         while !permit.is_revoked() {"] # [doc = "             // ..."] # [doc = " #           std::thread::sleep(core::time::Duration::from_millis(1));"] # [doc = "         }"] # [doc = "     });"] # [doc = " }"] # [doc = " wait_for_shutdown_signal();"] # [doc = " // Revoke all thread permits and wait for them to"] # [doc = " // finish and drop their permits."] # [doc = " top_permit"] # [doc = "     .revoke()"] # [doc = "     .wait_subs_timeout(core::time::Duration::from_secs(3))"] # [doc = "     .unwrap();"] # [doc = " ```"] pub struct Permit { node : Arc < Node > , }
};
}
