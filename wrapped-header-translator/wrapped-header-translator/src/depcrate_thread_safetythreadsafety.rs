// Generated macro for ThreadSafety (struct)
macro_rules! Depcrate_thread_safetyThreadSafety {
() => {
// Module: crate::thread_safety
// Provides: {"ThreadSafety"}
// Dependencies: {}
# [doc = " Information about thread-safety properties of a type."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct ThreadSafety { # [doc = " What the attribute was explicitly declared as."] explicit : Option < ThreadSafetyAttr > , # [doc = " What the attribute was inferred to be."] inferred : ThreadSafetyAttr , }
};
}
