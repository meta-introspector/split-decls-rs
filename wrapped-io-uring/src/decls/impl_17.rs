macro_rules! deps {
    () => {
        Mmap!();
        EntryMarker!();
        CompletionQueue!();
        Inner!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < E : EntryMarker > Inner < E > { # [rustfmt :: skip] pub (crate) unsafe fn new (cq_mmap : & Mmap , p : & sys :: io_uring_params) -> Self { let head = cq_mmap . offset (p . cq_off . head) as * const atomic :: AtomicU32 ; let tail = cq_mmap . offset (p . cq_off . tail) as * const atomic :: AtomicU32 ; let ring_mask = cq_mmap . offset (p . cq_off . ring_mask) . cast :: < u32 > () . read () ; let ring_entries = cq_mmap . offset (p . cq_off . ring_entries) . cast :: < u32 > () . read () ; let overflow = cq_mmap . offset (p . cq_off . overflow) as * const atomic :: AtomicU32 ; let cqes = cq_mmap . offset (p . cq_off . cqes) as * const E ; let flags = cq_mmap . offset (p . cq_off . flags) as * const atomic :: AtomicU32 ; Self { head , tail , ring_mask , ring_entries , overflow , cqes , flags , } } # [inline] pub (crate) unsafe fn borrow_shared (& self) -> CompletionQueue < '_ , E > { CompletionQueue { head : unsync_load (self . head) , tail : (* self . tail) . load (atomic :: Ordering :: Acquire) , queue : self , } } # [inline] pub (crate) fn borrow (& mut self) -> CompletionQueue < '_ , E > { unsafe { self . borrow_shared () } } }
    };
}

impl_17!();