macro_rules! deps {
    () => {
        BStr!();
    };
}

macro_rules! impl_131 {
    () => {
        deps!();
        impl Drop for BStr { fn drop (& mut self) { unsafe { SysFreeString (self . 0) } ; } }
    };
}

impl_131!()