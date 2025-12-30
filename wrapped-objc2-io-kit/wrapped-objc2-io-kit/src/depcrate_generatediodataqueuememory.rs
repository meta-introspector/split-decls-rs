// Generated macro for IODataQueueMemory (struct)
macro_rules! Depcrate_generatedIODataQueueMemory {
() => {
// Module: crate::generated
// Provides: {"IODataQueueMemory"}
// Dependencies: {}
# [doc = " A struct mapping to the header region of a data queue."] # [doc = ""] # [doc = " This struct is variable sized.  The struct represents the data queue header information plus a pointer to the actual data queue itself.  The size of the struct is the combined size of the header fields (3 * sizeof(UInt32)) plus the actual size of the queue region.  This size is stored in the queueSize field."] # [doc = " Field: queueSize The size of the queue region pointed to by the queue field."] # [doc = " Field: head The location of the queue head.  This field is represented as a byte offset from the beginning of the queue memory region."] # [doc = " Field: tail The location of the queue tail.  This field is represented as a byte offset from the beginning of the queue memory region."] # [doc = " Field: queue Represents the beginning of the queue memory region.  The size of the region pointed to by queue is stored in the queueSize field."] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/iokit/iodataqueuememory?language=objc)"] # [repr (C)] # [derive (Clone , Copy , Debug , PartialEq , Default)] pub struct IODataQueueMemory { pub queueSize : u32 , pub head : u32 , pub tail : u32 , pub queue : [IODataQueueEntry ; 1] , }
};
}
