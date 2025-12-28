macro_rules! deps {
    () => {
        Mem!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl Mem { # [inline] pub fn wipe < T : Default > (mut x : impl AsMut < [T] >) { let x = x . as_mut () ; for i in 0 .. x . len () { unsafe { ptr :: write_volatile (x . as_mut_ptr () . add (i) , T :: default ()) ; } } atomic :: compiler_fence (atomic :: Ordering :: SeqCst) ; atomic :: fence (atomic :: Ordering :: SeqCst) ; } }
    };
}

impl_8!();