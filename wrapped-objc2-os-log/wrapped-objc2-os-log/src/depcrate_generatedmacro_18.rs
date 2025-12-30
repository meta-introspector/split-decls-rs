// Generated macro for macro_18 (macro)
macro_rules! Depcrate_generatedmacro_18 {
() => {
// Module: crate::generated
// Provides: {"macro_18"}
// Dependencies: {}
extern_protocol ! (# [doc = " Entry subclasses conforming to this protocol represent data"] # [doc = " that are generated from a process; they have metadata about"] # [doc = " the originator."] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/oslog/oslogentryfromprocess?language=objc)"] pub unsafe trait OSLogEntryFromProcess { # [doc = " The activity ID associated with the entry."] # [unsafe (method (activityIdentifier))] # [unsafe (method_family = none)] unsafe fn activityIdentifier (& self) -> os_activity_id_t ; # [doc = " The name of the process that made the entry."] # [unsafe (method (process))] # [unsafe (method_family = none)] unsafe fn process (& self) -> Retained < NSString >; # [cfg (feature = "libc")] # [doc = " The pid of the process that made the entry."] # [unsafe (method (processIdentifier))] # [unsafe (method_family = none)] unsafe fn processIdentifier (& self) -> libc :: pid_t ; # [doc = " The name of the binary image that made the entry."] # [unsafe (method (sender))] # [unsafe (method_family = none)] unsafe fn sender (& self) -> Retained < NSString >; # [doc = " The tid of the thread that made the entry."] # [unsafe (method (threadIdentifier))] # [unsafe (method_family = none)] unsafe fn threadIdentifier (& self) -> u64 ; }) ;
};
}
