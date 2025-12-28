macro_rules! have_elision {
    () => {
        # [inline] pub fn have_elision () -> bool { cfg ! (all (feature = "hardware-lock-elision" , not (miri) , any (target_arch = "x86" , target_arch = "x86_64") ,)) }
    };
}

have_elision!()