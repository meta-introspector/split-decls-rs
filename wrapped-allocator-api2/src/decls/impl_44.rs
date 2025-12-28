macro_rules! deps {
    () => {
        Box!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        # [cfg (not (no_global_oom_handling))] impl < T , const N : usize > From < [T ; N] > for Box < [T] > { # [doc = " Converts a `[T; N]` into a `Box<[T]>`"] # [doc = ""] # [doc = " This conversion moves the array to newly heap-allocated memory."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use allocator_api2::boxed::Box;"] # [doc = ""] # [doc = " let boxed: Box<[u8]> = Box::from([4, 2]);"] # [doc = " println!(\"{boxed:?}\");"] # [doc = " ```"] # [inline (always)] fn from (array : [T ; N]) -> Box < [T] > { Box :: slice (Box :: new (array)) } }
    };
}

impl_44!()