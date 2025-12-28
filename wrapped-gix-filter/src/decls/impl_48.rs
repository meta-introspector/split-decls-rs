macro_rules! deps {
    () => {
        WriterThread!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl Drop for WriterThread { fn drop (& mut self) { if let Err (_err) = self . join () { gix_trace :: debug ! (err = % _err , "Failed to join writer thread during drop") ; } } }
    };
}

impl_48!()