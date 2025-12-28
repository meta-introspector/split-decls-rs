macro_rules! deps {
    () => {
        BlobRef!();
        Blob!();
    };
}

macro_rules! impl_150 {
    () => {
        deps!();
        impl BlobRef < '_ > { # [doc = " Instantiate a `Blob` from the given `data`, which is used as-is."] pub fn from_bytes (data : & [u8]) -> Result < BlobRef < '_ > , Infallible > { Ok (BlobRef { data }) } # [doc = " Clone the data in this instance by allocating a new vector for a fully owned blob."] pub fn into_owned (self) -> Blob { self . into () } }
    };
}

impl_150!();