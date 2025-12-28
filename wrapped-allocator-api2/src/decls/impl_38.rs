macro_rules! deps {
    () => {
        Box!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        # [cfg (not (no_global_oom_handling))] impl < T > From < T > for Box < T > { # [doc = " Converts a `T` into a `Box<T>`"] # [doc = ""] # [doc = " The conversion allocates on the heap and moves `t`"] # [doc = " from the stack into it."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use allocator_api2::boxed::Box;"] # [doc = ""] # [doc = " let x = 5;"] # [doc = " let boxed = Box::new(5);"] # [doc = ""] # [doc = " assert_eq!(Box::from(x), boxed);"] # [doc = " ```"] # [inline (always)] fn from (t : T) -> Self { Box :: new (t) } }
    };
}

impl_38!();