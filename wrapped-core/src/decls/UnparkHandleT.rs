macro_rules! deps {
    () => {
        ThreadParkerT!();
    };
}

macro_rules! UnparkHandleT {
    () => {
        deps!();
        # [doc = " Handle for a thread that is about to be unparked. We need to mark the thread"] # [doc = " as unparked while holding the queue lock, but we delay the actual unparking"] # [doc = " until after the queue lock is released."] pub trait UnparkHandleT { # [doc = " Wakes up the parked thread. This should be called after the queue lock is"] # [doc = " released to avoid blocking the queue for too long."] # [doc = ""] # [doc = " This method is unsafe for the same reason as the unsafe methods in"] # [doc = " `ThreadParkerT`."] unsafe fn unpark (self) ; }
    };
}

UnparkHandleT!();