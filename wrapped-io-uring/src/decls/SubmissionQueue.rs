macro_rules! deps {
    () => {
        Entry!();
        EntryMarker!();
        Inner!();
    };
}

macro_rules! SubmissionQueue {
    () => {
        deps!();
        # [doc = " An io_uring instance's submission queue. This is used to send I/O requests to the kernel."] pub struct SubmissionQueue < 'a , E : EntryMarker = Entry > { head : u32 , tail : u32 , queue : & 'a Inner < E > , }
    };
}

SubmissionQueue!();