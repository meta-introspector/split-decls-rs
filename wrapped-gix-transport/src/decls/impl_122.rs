macro_rules! deps {
    () => {
        RequestWriter!();
    };
}

macro_rules! impl_122 {
    () => {
        deps!();
        impl io :: Write for RequestWriter < '_ > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { # [allow (unused_imports)] if self . trace { use bstr :: ByteSlice ; gix_features :: trace :: trace ! (">> {}" , buf . as_bstr ()) ; } self . writer . write (buf) } fn flush (& mut self) -> io :: Result < () > { self . writer . flush () } }
    };
}

impl_122!();