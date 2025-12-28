macro_rules! deps {
    () => {
        SubmissionQueue!();
        Inner!();
        Mmap!();
        EntryMarker!();
    };
}

macro_rules! impl_135 {
    () => {
        deps!();
        impl < E : EntryMarker > Inner < E > { # [rustfmt :: skip] pub (crate) unsafe fn new (sq_mmap : & Mmap , sqe_mmap : & Mmap , p : & sys :: io_uring_params ,) -> Self { let head = sq_mmap . offset (p . sq_off . head) as * const atomic :: AtomicU32 ; let tail = sq_mmap . offset (p . sq_off . tail) as * const atomic :: AtomicU32 ; let ring_mask = sq_mmap . offset (p . sq_off . ring_mask) . cast :: < u32 > () . read () ; let ring_entries = sq_mmap . offset (p . sq_off . ring_entries) . cast :: < u32 > () . read () ; let flags = sq_mmap . offset (p . sq_off . flags) as * const atomic :: AtomicU32 ; let dropped = sq_mmap . offset (p . sq_off . dropped) as * const atomic :: AtomicU32 ; let array = sq_mmap . offset (p . sq_off . array) as * mut u32 ; let sqes = sqe_mmap . as_mut_ptr () as * mut E ; for i in 0 .. ring_entries { array . add (i as usize) . write_volatile (i) ; } Self { head , tail , ring_mask , ring_entries , flags , dropped , sqes , } } # [inline] pub (crate) unsafe fn borrow_shared (& self) -> SubmissionQueue < '_ , E > { SubmissionQueue { head : (* self . head) . load (atomic :: Ordering :: Acquire) , tail : unsync_load (self . tail) , queue : self , } } # [inline] pub (crate) fn borrow (& mut self) -> SubmissionQueue < '_ , E > { unsafe { self . borrow_shared () } } }
    };
}

impl_135!()