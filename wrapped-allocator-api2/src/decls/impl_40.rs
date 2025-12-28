macro_rules! deps {
    () => {
        RawVec!();
        Box!();
        Allocator!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        # [cfg (not (no_global_oom_handling))] impl < T : Copy , A : Allocator + Default > From < & [T] > for Box < [T] , A > { # [doc = " Converts a `&[T]` into a `Box<[T]>`"] # [doc = ""] # [doc = " This conversion allocates on the heap"] # [doc = " and performs a copy of `slice` and its contents."] # [doc = ""] # [doc = " # Examples"] # [doc = " ```"] # [doc = " use allocator_api2::boxed::Box;"] # [doc = ""] # [doc = " // create a &[u8] which will be used to create a Box<[u8]>"] # [doc = " let slice: &[u8] = &[104, 101, 108, 108, 111];"] # [doc = " let boxed_slice: Box<[u8]> = Box::from(slice);"] # [doc = ""] # [doc = " println!(\"{boxed_slice:?}\");"] # [doc = " ```"] # [inline (always)] fn from (slice : & [T]) -> Box < [T] , A > { let len = slice . len () ; let buf = RawVec :: with_capacity_in (len , A :: default ()) ; unsafe { ptr :: copy_nonoverlapping (slice . as_ptr () , buf . ptr () , len) ; buf . into_box (slice . len ()) . assume_init () } } }
    };
}

impl_40!();