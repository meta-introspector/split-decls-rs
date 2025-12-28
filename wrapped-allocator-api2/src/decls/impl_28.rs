macro_rules! deps {
    () => {
        Allocator!();
        Box!();
        Unique!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < T , A : Allocator + Default > Default for Box < [T] , A > { # [inline (always)] fn default () -> Self { let ptr : NonNull < [T] > = NonNull :: < [T ; 0] > :: dangling () ; Box (unsafe { Unique :: new_unchecked (ptr . as_ptr ()) } , A :: default ()) } }
    };
}

impl_28!()