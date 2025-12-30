// Generated macro for JMethodID (struct)
macro_rules! Depcrate_ids_jmethodidJMethodID {
() => {
// Module: crate::ids::jmethodid
// Provides: {"JMethodID"}
// Dependencies: {}
# [doc = " Wrapper around [`jmethodID`] that implements `Send` + `Sync` since method IDs"] # [doc = " are valid across threads (not tied to a `Env`)."] # [doc = ""] # [doc = " There is no lifetime associated with these since they aren't garbage"] # [doc = " collected like objects and their lifetime is not implicitly connected with"] # [doc = " the scope in which they are queried."] # [doc = ""] # [doc = " It matches C's representation of the raw pointer, so it can be used in any"] # [doc = " of the extern function argument positions that would take a [`jmethodID`]."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " According to the JNI spec method IDs may be invalidated when the"] # [doc = " corresponding class is unloaded."] # [doc = ""] # [doc = " Since this constraint can't be encoded as a Rust lifetime, and to avoid the"] # [doc = " excessive cost of having every Method ID be associated with a global"] # [doc = " reference to the corresponding class then it is the developers"] # [doc = " responsibility to ensure they hold some class reference for the lifetime of"] # [doc = " cached method IDs."] # [repr (transparent)] # [derive (Copy , Clone , Debug)] pub struct JMethodID { internal : jmethodID , }
};
}
