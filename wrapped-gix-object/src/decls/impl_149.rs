macro_rules! deps {
    () => {
        Blob!();
        BlobRef!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        impl Blob { # [doc = " Provide a `BlobRef` to this owned blob"] pub fn to_ref (& self) -> BlobRef < '_ > { BlobRef { data : & self . data } } }
    };
}

impl_149!()