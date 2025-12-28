macro_rules! BufRingEntry {
    () => {
        # [repr (transparent)] pub struct BufRingEntry (sys :: io_uring_buf) ;
    };
}

BufRingEntry!();