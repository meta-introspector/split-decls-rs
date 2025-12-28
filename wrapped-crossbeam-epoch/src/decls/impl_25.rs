macro_rules! deps {
    () => {
        Atomic!();
        Pointable!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < T : ? Sized + Pointable > Clone for Atomic < T > { # [doc = " Returns a copy of the atomic value."] # [doc = ""] # [doc = " Note that a `Relaxed` load is used here. If you need synchronization, use it with other"] # [doc = " atomics or fences."] fn clone (& self) -> Self { let data = self . data . load (Ordering :: Relaxed) ; Self :: from_ptr (data) } }
    };
}

impl_25!();