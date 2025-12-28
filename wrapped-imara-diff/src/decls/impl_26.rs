macro_rules! deps {
    () => {
        Myers!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl Drop for Myers { fn drop (& mut self) { unsafe { drop (Box :: from_raw (self . kvec . as_ptr ())) } } }
    };
}

impl_26!();