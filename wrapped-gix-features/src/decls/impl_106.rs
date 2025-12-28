macro_rules! deps {
    () => {
        Compress!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl Drop for Compress { fn drop (& mut self) { unsafe { libz_rs_sys :: deflateEnd (& mut self . 0) } ; } }
    };
}

impl_106!();