// Generated macro for other_4889 (other)
macro_rules! Depcrate_generatedother_4889 {
() => {
// Module: crate::generated
// Provides: {"other_4889"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Used to peek at the next entry on the queue."] # [doc = ""] # [doc = " This function can be used to look at the next entry which allows the entry to be received without having to copy it with IODataQueueDequeue.  In order to do this, call IODataQueuePeek to get the entry.  Then call IODataQueueDequeue with a NULL data pointer.  That will cause the head to be moved to the next entry, but no memory to be copied."] # [doc = ""] # [doc = " Parameter `dataQueue`: The IODataQueueMemory region mapped from the kernel."] # [doc = ""] # [doc = " Returns: Returns a pointer to the next IODataQueueEntry if one is available.  Zero is returned if the queue is empty."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `data_queue` must be a valid pointer."] pub fn IODataQueuePeek (data_queue : * mut IODataQueueMemory) -> * mut IODataQueueEntry ; }
};
}
