macro_rules! deps {
    () => {
        Decompress!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        impl Drop for Decompress { fn drop (& mut self) { unsafe { libz_rs_sys :: inflateEnd (& mut self . 0) } ; } }
    };
}

impl_91!()