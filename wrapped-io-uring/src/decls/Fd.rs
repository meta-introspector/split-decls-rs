macro_rules! Fd {
    () => {
        # [doc = " A file descriptor that has not been registered with io_uring."] # [derive (Debug , Clone , Copy)] # [repr (transparent)] pub struct Fd (pub RawFd) ;
    };
}

Fd!()