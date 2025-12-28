macro_rules! deps {
    () => {
        Shallow!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl Shallow { # [doc = " Produce a variant that causes the repository to loose its shallow boundary, effectively by extending it"] # [doc = " beyond all limits."] pub fn undo () -> Self { Shallow :: DepthAtRemote ((i32 :: MAX as u32) . try_into () . expect ("valid at compile time")) } }
    };
}

impl_67!()