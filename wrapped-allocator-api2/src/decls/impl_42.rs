macro_rules! deps {
    () => {
        Box!();
        Allocator!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl < A : Allocator > From < Box < str , A > > for Box < [u8] , A > { # [doc = " Converts a `Box<str>` into a `Box<[u8]>`"] # [doc = ""] # [doc = " This conversion does not allocate on the heap and happens in place."] # [doc = ""] # [doc = " # Examples"] # [doc = " ```"] # [doc = " use allocator_api2::boxed::Box;"] # [doc = ""] # [doc = " // create a Box<str> which will be used to create a Box<[u8]>"] # [doc = " let boxed: Box<str> = Box::from(\"hello\");"] # [doc = " let boxed_str: Box<[u8]> = Box::from(boxed);"] # [doc = ""] # [doc = " // create a &[u8] which will be used to create a Box<[u8]>"] # [doc = " let slice: &[u8] = &[104, 101, 108, 108, 111];"] # [doc = " let boxed_slice = Box::from(slice);"] # [doc = ""] # [doc = " assert_eq!(boxed_slice, boxed_str);"] # [doc = " ```"] # [inline (always)] fn from (s : Box < str , A >) -> Self { let (raw , alloc) = Box :: into_raw_with_allocator (s) ; unsafe { Box :: from_raw_in (raw as * mut [u8] , alloc) } } }
    };
}

impl_42!();