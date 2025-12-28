macro_rules! deps {
    () => {
        RawTable!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl < T , A : Allocator + Default > Default for RawTable < T , A > { # [inline] fn default () -> Self { Self :: new_in (Default :: default ()) } }
    };
}

impl_64!()