macro_rules! deps {
    () => {
        Accels!();
        Accel!();
    };
}

macro_rules! AccelTy {
    () => {
        deps!();
        # [doc = " The base type used to represent a collection of accelerators."] # [doc = ""] # [doc = " While an `Accel` is represented as a fixed size array of bytes, a"] # [doc = " *collection* of `Accel`s (called `Accels`) is represented internally as a"] # [doc = " slice of u32. While it's a bit unnatural to do this and costs us a bit of"] # [doc = " fairly low-risk not-safe code, it lets us remove the need for a second type"] # [doc = " parameter in the definition of dense::DFA. (Which really wants everything"] # [doc = " to be a slice of u32.)"] type AccelTy = u32 ;
    };
}

AccelTy!()