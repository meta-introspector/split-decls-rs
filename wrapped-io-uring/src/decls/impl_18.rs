macro_rules! deps {
    () => {
        CompletionQueue!();
        EntryMarker!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl < E : EntryMarker > CompletionQueue < '_ , E > { # [doc = " Synchronize this type with the real completion queue."] # [doc = ""] # [doc = " This will flush any entries consumed in this iterator and will make available new entries"] # [doc = " in the queue if the kernel has produced some entries in the meantime."] # [inline] pub fn sync (& mut self) { unsafe { (* self . queue . head) . store (self . head , atomic :: Ordering :: Release) ; self . tail = (* self . queue . tail) . load (atomic :: Ordering :: Acquire) ; } } # [doc = " If queue is full and [`is_feature_nodrop`](crate::Parameters::is_feature_nodrop) is not set,"] # [doc = " new events may be dropped. This records the number of dropped events."] pub fn overflow (& self) -> u32 { unsafe { (* self . queue . overflow) . load (atomic :: Ordering :: Acquire) } } # [doc = " Whether eventfd notifications are disabled when a request is completed and queued to the CQ"] # [doc = " ring. This library currently does not provide a way to set it, so this will always be"] # [doc = " `false`."] pub fn eventfd_disabled (& self) -> bool { unsafe { (* self . queue . flags) . load (atomic :: Ordering :: Acquire) & sys :: IORING_CQ_EVENTFD_DISABLED != 0 } } # [doc = " Get the total number of entries in the completion queue ring buffer."] # [inline] pub fn capacity (& self) -> usize { self . queue . ring_entries as usize } # [doc = " Returns `true` if there are no completion queue events to be processed."] # [inline] pub fn is_empty (& self) -> bool { self . len () == 0 } # [doc = " Returns `true` if the completion queue is at maximum capacity. If"] # [doc = " [`is_feature_nodrop`](crate::Parameters::is_feature_nodrop) is not set, this will cause any"] # [doc = " new completion queue events to be dropped by the kernel."] # [inline] pub fn is_full (& self) -> bool { self . len () == self . capacity () } # [inline] pub fn fill < 'a > (& mut self , entries : & 'a mut [MaybeUninit < E >]) -> & 'a mut [E] { let len = std :: cmp :: min (self . len () , entries . len ()) ; for entry in & mut entries [.. len] { entry . write (unsafe { self . pop () }) ; } unsafe { std :: slice :: from_raw_parts_mut (entries as * mut _ as * mut E , len) } } # [inline] unsafe fn pop (& mut self) -> E { let entry = & * self . queue . cqes . add ((self . head & self . queue . ring_mask) as usize) ; self . head = self . head . wrapping_add (1) ; entry . clone () } }
    };
}

impl_18!()