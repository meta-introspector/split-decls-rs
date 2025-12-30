// Generated macro for ThreadSafetyAttr (enum)
macro_rules! Depcrate_thread_safetyThreadSafetyAttr {
() => {
// Module: crate::thread_safety
// Provides: {"ThreadSafetyAttr"}
// Dependencies: {}
# [derive (Debug , Clone , Copy , PartialEq , Eq , Hash , Deserialize)] pub (crate) enum ThreadSafetyAttr { # [doc = " The item is only accessible from the main thread."] # [doc = ""] # [doc = " This implies that the item is not sendable."] MainThreadOnly , # [doc = " The item is sendable."] Sendable , # [doc = " The item is not sendable."] NotSendable , }
};
}
