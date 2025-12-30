// Generated macro for other_4890 (other)
macro_rules! Depcrate_generatedother_4890 {
() => {
// Module: crate::generated
// Provides: {"other_4890"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Dequeues the next available entry on the queue and copies it into the given data pointer."] # [doc = ""] # [doc = " This function will dequeue the next available entry on the queue.  If a data pointer is provided, it will copy the data into the memory region if there is enough space available as specified in the dataSize parameter.  If no data pointer is provided, it will simply move the head value past the current entry."] # [doc = ""] # [doc = " Parameter `dataQueue`: The IODataQueueMemory region mapped from the kernel."] # [doc = ""] # [doc = " Parameter `data`: A pointer to the data memory region in which to copy the next entry data on the queue.  If this parameter is 0 (NULL), it will simply move to the next entry."] # [doc = ""] # [doc = " Parameter `dataSize`: A pointer to the size of the data parameter.  On return, this contains the size of the actual entry data - even if the original size was not large enough."] # [doc = ""] # [doc = " Returns: Returns kIOReturnSuccess on success.  Other return values possible are: kIOReturnUnderrun - queue is empty, kIOReturnBadArgument - no dataQueue or no dataSize, kIOReturnNoSpace - dataSize is too small for entry."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `data_queue` must be a valid pointer."] # [doc = " - `data` must be a valid pointer."] # [doc = " - `data_size` must be a valid pointer."] pub fn IODataQueueDequeue (data_queue : * mut IODataQueueMemory , data : * mut c_void , data_size : * mut u32 ,) -> IOReturn ; }
};
}
