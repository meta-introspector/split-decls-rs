macro_rules! deps {
    () => {
        Parameters!();
    };
}

macro_rules! Submitter {
    () => {
        deps!();
        # [doc = " Interface for submitting submission queue events in an io_uring instance to the kernel for"] # [doc = " executing and registering files or buffers with the instance."] # [doc = ""] # [doc = " io_uring supports both directly performing I/O on buffers and file descriptors and registering"] # [doc = " them beforehand. Registering is slow, but it makes performing the actual I/O much faster."] pub struct Submitter < 'a > { fd : & 'a OwnedFd , params : & 'a Parameters , sq_head : * const atomic :: AtomicU32 , sq_tail : * const atomic :: AtomicU32 , sq_flags : * const atomic :: AtomicU32 , }
    };
}

Submitter!();