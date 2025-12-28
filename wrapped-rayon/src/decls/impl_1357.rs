macro_rules! deps {
    () => {
        DrainProducer!();
    };
}

macro_rules! impl_1357 {
    () => {
        deps!();
        impl < T : Send > DrainProducer < '_ , T > { # [doc = " Creates a draining producer, which *moves* items from the slice."] # [doc = ""] # [doc = " Unsafe because `!Copy` data must not be read after the borrow is released."] pub (crate) unsafe fn new (slice : & mut [T]) -> DrainProducer < '_ , T > { DrainProducer { slice } } # [doc = " Creates a draining producer, which *moves* items from the tail of the vector."] # [doc = ""] # [doc = " Unsafe because we're moving from beyond `vec.len()`, so the caller must ensure"] # [doc = " that data is initialized and not read after the borrow is released."] unsafe fn from_vec (vec : & mut Vec < T > , len : usize) -> DrainProducer < '_ , T > { unsafe { let start = vec . len () ; assert ! (vec . capacity () - start >= len) ; let ptr = vec . as_mut_ptr () . add (start) ; DrainProducer :: new (slice :: from_raw_parts_mut (ptr , len)) } } }
    };
}

impl_1357!()