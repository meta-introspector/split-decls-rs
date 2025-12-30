// Generated macro for DispatchAutoReleaseFrequency (struct)
macro_rules! Depcrate_generatedDispatchAutoReleaseFrequency {
() => {
// Module: crate::generated
// Provides: {"DispatchAutoReleaseFrequency"}
// Dependencies: {}
# [doc = " Values to pass to the dispatch_queue_attr_make_with_autorelease_frequency()"] # [doc = " function."] # [doc = ""] # [doc = ""] # [doc = " Dispatch queues with this autorelease frequency inherit the behavior from"] # [doc = " their target queue. This is the default behavior for manually created queues."] # [doc = ""] # [doc = ""] # [doc = " Dispatch queues with this autorelease frequency push and pop an autorelease"] # [doc = " pool around the execution of every block that was submitted to it"] # [doc = " asynchronously."] # [doc = ""] # [doc = " See: dispatch_queue_attr_make_with_autorelease_frequency()."] # [doc = ""] # [doc = ""] # [doc = " Dispatch queues with this autorelease frequency never set up an individual"] # [doc = " autorelease pool around the execution of a block that is submitted to it"] # [doc = " asynchronously. This is the behavior of the global concurrent queues."] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/dispatch/dispatchautoreleasefrequency?language=objc)"] # [doc (alias = "dispatch_autorelease_frequency_t")] # [repr (transparent)] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash , PartialOrd , Ord , Default)] pub struct DispatchAutoReleaseFrequency (pub c_ulong) ;
};
}
