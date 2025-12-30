// Generated macro for Inner (struct)
macro_rules! DepcrateInner {
() => {
// Module: crate
// Provides: {"Inner"}
// Dependencies: {}
# [doc = " Inner state of [`Event`]."] struct Inner < T > { # [doc = " The number of notified entries, or `usize::MAX` if all of them have been notified."] # [doc = ""] # [doc = " If there are no entries, this value is set to `usize::MAX`."] notified : AtomicUsize , # [doc = " Inner queue of event listeners."] # [doc = ""] # [doc = " On `std` platforms, this is an intrusive linked list. On `no_std` platforms, this is a"] # [doc = " more traditional `Vec` of listeners, with an atomic queue used as a backup for high"] # [doc = " contention."] list : sys :: List < T > , }
};
}
