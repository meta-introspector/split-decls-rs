macro_rules! deps {
    () => {
        Allocator!();
        Vec!();
        Global!();
    };
}

macro_rules! Drain {
    () => {
        deps!();
        # [doc = " A draining iterator for `Vec<T>`."] # [doc = ""] # [doc = " This `struct` is created by [`Vec::drain`]."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use allocator_api2::vec;"] # [doc = ""] # [doc = " let mut v = vec![0, 1, 2];"] # [doc = " let iter: vec::Drain<_> = v.drain(..);"] # [doc = " ```"] pub struct Drain < 'a , T : 'a , A : Allocator + 'a = Global > { # [doc = " Index of tail to preserve"] pub (super) tail_start : usize , # [doc = " Length of tail"] pub (super) tail_len : usize , # [doc = " Current remaining range to remove"] pub (super) iter : slice :: Iter < 'a , T > , pub (super) vec : NonNull < Vec < T , A > > , }
    };
}

Drain!()