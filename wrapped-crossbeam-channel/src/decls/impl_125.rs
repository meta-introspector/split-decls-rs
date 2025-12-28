macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! impl_125 {
    () => {
        deps!();
        impl < T > Block < T > { const LAYOUT : Layout = { let layout = Layout :: new :: < Self > () ; assert ! (layout . size () != 0 , "Block should never be zero-sized, as it has an AtomicPtr field") ; layout } ; # [doc = " Creates an empty block."] fn new () -> Box < Self > { let ptr = unsafe { alloc_zeroed (Self :: LAYOUT) } ; if ptr . is_null () { handle_alloc_error (Self :: LAYOUT) } unsafe { Box :: from_raw (ptr . cast ()) } } # [doc = " Waits until the next pointer is set."] fn wait_next (& self) -> * mut Self { let backoff = Backoff :: new () ; loop { let next = self . next . load (Ordering :: Acquire) ; if ! next . is_null () { return next ; } backoff . snooze () ; } } # [doc = " Sets the `DESTROY` bit in slots starting from `start` and destroys the block."] unsafe fn destroy (this : * mut Self , start : usize) { for i in start .. BLOCK_CAP - 1 { let slot = unsafe { (* this) . slots . get_unchecked (i) } ; if slot . state . load (Ordering :: Acquire) & READ == 0 && slot . state . fetch_or (DESTROY , Ordering :: AcqRel) & READ == 0 { return ; } } drop (unsafe { Box :: from_raw (this) }) ; } }
    };
}

impl_125!()