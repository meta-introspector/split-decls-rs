macro_rules! deps {
    () => {
        ReadBytesExt!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        # [doc = " All types that implement `Read` get methods defined in `ReadBytesExt`"] # [doc = " for free."] impl < R : io :: Read + ? Sized > ReadBytesExt for R { }
    };
}

impl_1!()