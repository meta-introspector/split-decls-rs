macro_rules! deps {
    () => {
        EntryMarker!();
        Entry!();
        Inner!();
    };
}

macro_rules! CompletionQueue {
    () => {
        deps!();
        # [doc = " An io_uring instance's completion queue. This stores all the I/O operations that have completed."] pub struct CompletionQueue < 'a , E : EntryMarker = Entry > { head : u32 , tail : u32 , queue : & 'a Inner < E > , }
    };
}

CompletionQueue!()