macro_rules! deps {
    () => {
        Entry!();
        Parameters!();
        Builder!();
        IoUring!();
    };
}

macro_rules! impl_209 {
    () => {
        deps!();
        impl IoUring < squeue :: Entry , cqueue :: Entry > { # [doc = " Create a new `IoUring` instance with default configuration parameters. See [`Builder`] to"] # [doc = " customize it further."] # [doc = ""] # [doc = " The `entries` sets the size of queue,"] # [doc = " and its value should be the power of two."] pub fn new (entries : u32) -> io :: Result < Self > { Self :: builder () . build (entries) } # [doc = " Create an `IoUring` instance from a pre-opened file descriptor."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The caller must uphold that the file descriptor is owned and refers to a uring. The"] # [doc = " `params` argument must be equivalent to the those previously filled in by the kernel when"] # [doc = " the provided ring was created."] pub unsafe fn from_fd (fd : RawFd , params : Parameters) -> io :: Result < Self > { Self :: with_fd_and_params (OwnedFd :: from_raw_fd (fd) , params . 0) } }
    };
}

impl_209!()