// Generated macro for Inner (struct)
macro_rules! Depcrate_dequeInner {
() => {
// Module: crate::deque
// Provides: {"Inner"}
// Dependencies: {}
# [doc = " Internal queue data shared between the worker and stealers."] # [doc = ""] # [doc = " The implementation is based on the following work:"] # [doc = ""] # [doc = " 1. [Chase and Lev. Dynamic circular work-stealing deque. SPAA 2005.][chase-lev]"] # [doc = " 2. [Le, Pop, Cohen, and Nardelli. Correct and efficient work-stealing for weak memory models."] # [doc = "    PPoPP 2013.][weak-mem]"] # [doc = " 3. [Norris and Demsky. CDSchecker: checking concurrent data structures written with C/C++"] # [doc = "    atomics. OOPSLA 2013.][checker]"] # [doc = ""] # [doc = " [chase-lev]: https://dl.acm.org/citation.cfm?id=1073974"] # [doc = " [weak-mem]: https://dl.acm.org/citation.cfm?id=2442524"] # [doc = " [checker]: https://dl.acm.org/citation.cfm?id=2509514"] struct Inner < T > { # [doc = " The front index."] front : AtomicIsize , # [doc = " The back index."] back : AtomicIsize , # [doc = " The underlying buffer."] buffer : CachePadded < Atomic < Buffer < T > > > , }
};
}
