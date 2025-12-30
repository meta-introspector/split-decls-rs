// Generated macro for Buffer (struct)
macro_rules! Depcrate_dequeBuffer {
() => {
// Module: crate::deque
// Provides: {"Buffer"}
// Dependencies: {}
# [doc = " A buffer that holds tasks in a worker queue."] # [doc = ""] # [doc = " This is just a pointer to the buffer and its length - dropping an instance of this struct will"] # [doc = " *not* deallocate the buffer."] struct Buffer < T > { # [doc = " Pointer to the allocated memory."] ptr : * mut T , # [doc = " Capacity of the buffer. Always a power of two."] cap : usize , }
};
}
