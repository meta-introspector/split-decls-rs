macro_rules! deps {
    () => {
        FuseState!();
    };
}

macro_rules! Fuse {
    () => {
        deps!();
        # [doc = " An iterator which is well-behaved at the beginning and end of iteration."] pub struct Fuse < I > { it : I , state : FuseState , }
    };
}

Fuse!();