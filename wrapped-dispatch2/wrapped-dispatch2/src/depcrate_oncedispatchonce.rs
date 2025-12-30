// Generated macro for DispatchOnce (struct)
macro_rules! Depcrate_onceDispatchOnce {
() => {
// Module: crate::once
// Provides: {"DispatchOnce"}
// Dependencies: {}
# [doc = " A low-level synchronization primitive for one-time global execution."] # [doc = ""] # [doc = " This is equivalent to [`std::sync::Once`], except that this uses the"] # [doc = " underlying system primitives from `libdispatch`, which:"] # [doc = " - Might result in less code-size overhead."] # [doc = " - Aborts on panics in the initialization closure."] # [doc = ""] # [doc = " Generally, prefer [`std::sync::Once`] unless you have a specific need for"] # [doc = " this."] # [doc = ""] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " Run a closure once for the duration of the entire program, without using"] # [doc = " [`std::sync::Once`]."] # [doc = ""] # [doc = " ```"] # [doc = " use dispatch2::DispatchOnce;"] # [doc = ""] # [doc = " static INIT: DispatchOnce = DispatchOnce::new();"] # [doc = ""] # [doc = " INIT.call_once(|| {"] # [doc = "     // run initialization here"] # [doc = " });"] # [doc = " ```"] # [doc = ""] # [cfg_attr (not (feature = "std") , doc = "[`std::sync::Once`]: #std-not-enabled")] # [doc (alias = "dispatch_once_t")] pub struct DispatchOnce { predicate : UnsafeCell < dispatch_once_t > , }
};
}
