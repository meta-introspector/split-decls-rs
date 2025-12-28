macro_rules! deps {
    () => {
        Box!();
        Allocator!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        # [cfg (not (no_global_oom_handling))] impl < A : Allocator + Default > From < & str > for Box < str , A > { # [doc = " Converts a `&str` into a `Box<str>`"] # [doc = ""] # [doc = " This conversion allocates on the heap"] # [doc = " and performs a copy of `s`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use allocator_api2::boxed::Box;"] # [doc = ""] # [doc = " let boxed: Box<str> = Box::from(\"hello\");"] # [doc = " println!(\"{boxed}\");"] # [doc = " ```"] # [inline (always)] fn from (s : & str) -> Box < str , A > { let (raw , alloc) = Box :: into_raw_with_allocator (Box :: < [u8] , A > :: from (s . as_bytes ())) ; unsafe { Box :: from_raw_in (raw as * mut str , alloc) } } }
    };
}

impl_41!();