macro_rules! deps {
    () => {
        DHOutput!();
    };
}

macro_rules! impl_132 {
    () => {
        deps!();
        impl Deref for DHOutput { type Target = [u8 ; DHOutput :: BYTES] ; # [doc = " Returns the output of the scalar multiplication as bytes."] # [doc = " The output is not uniform, and should be hashed before use."] fn deref (& self) -> & Self :: Target { & self . 0 } }
    };
}

impl_132!();