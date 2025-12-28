macro_rules! deps {
    () => {
        WriteBytesExt!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        # [doc = " All types that implement `Write` get methods defined in `WriteBytesExt`"] # [doc = " for free."] impl < W : io :: Write + ? Sized > WriteBytesExt for W { }
    };
}

impl_3!()