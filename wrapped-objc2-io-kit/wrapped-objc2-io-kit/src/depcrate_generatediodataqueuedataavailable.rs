// Generated macro for IODataQueueDataAvailable (function)
macro_rules! Depcrate_generatedIODataQueueDataAvailable {
() => {
// Module: crate::generated
// Provides: {"IODataQueueDataAvailable"}
// Dependencies: {}
# [doc = " Used to determine if more data is available on the queue."] # [doc = ""] # [doc = " Parameter `dataQueue`: The IODataQueueMemory region mapped from the kernel."] # [doc = ""] # [doc = " Returns: Returns true if data is available and false if not."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `data_queue` must be a valid pointer."] # [inline] pub unsafe extern "C-unwind" fn IODataQueueDataAvailable (data_queue : * mut IODataQueueMemory ,) -> bool { extern "C-unwind" { fn IODataQueueDataAvailable (data_queue : * mut IODataQueueMemory) -> Boolean ; } let ret = unsafe { IODataQueueDataAvailable (data_queue) } ; ret != 0 }
};
}
