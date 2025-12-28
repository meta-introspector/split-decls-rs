macro_rules! deps {
    () => {
        HashBuffers!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl HashBuffers { # [inline] pub fn reset (& mut self) { self . dict . fill (0) ; self . next . fill (0) ; self . hash . fill (0) ; } }
    };
}

impl_10!();