macro_rules! deps {
    () => {
        DHOutput!();
    };
}

macro_rules! impl_133 {
    () => {
        deps!();
        impl DerefMut for DHOutput { # [doc = " Returns the output of the scalar multiplication as bytes."] # [doc = " The output is not uniform, and should be hashed before use."] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } }
    };
}

impl_133!();