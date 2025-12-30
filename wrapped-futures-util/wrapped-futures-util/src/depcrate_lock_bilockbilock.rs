// Generated macro for BiLock (struct)
macro_rules! Depcrate_lock_bilockBiLock {
() => {
// Module: crate::lock::bilock
// Provides: {"BiLock"}
// Dependencies: {}
# [doc = " A type of futures-powered synchronization primitive which is a mutex between"] # [doc = " two possible owners."] # [doc = ""] # [doc = " This primitive is not as generic as a full-blown mutex but is sufficient for"] # [doc = " many use cases where there are only two possible owners of a resource. The"] # [doc = " implementation of `BiLock` can be more optimized for just the two possible"] # [doc = " owners."] # [doc = ""] # [doc = " Note that it's possible to use this lock through a poll-style interface with"] # [doc = " the `poll_lock` method but you can also use it as a future with the `lock`"] # [doc = " method that consumes a `BiLock` and returns a future that will resolve when"] # [doc = " it's locked."] # [doc = ""] # [doc = " A `BiLock` is typically used for \"split\" operations where data which serves"] # [doc = " two purposes wants to be split into two to be worked with separately. For"] # [doc = " example a TCP stream could be both a reader and a writer or a framing layer"] # [doc = " could be both a stream and a sink for messages. A `BiLock` enables splitting"] # [doc = " these two and then using each independently in a futures-powered fashion."] # [doc = ""] # [doc = " This type is only available when the `bilock` feature of this"] # [doc = " library is activated."] # [derive (Debug)] # [cfg_attr (docsrs , doc (cfg (feature = "bilock")))] pub struct BiLock < T > { arc : Arc < Inner < T > > , }
};
}
