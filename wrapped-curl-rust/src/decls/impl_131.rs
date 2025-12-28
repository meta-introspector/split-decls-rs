macro_rules! deps {
    () => {
        RawMulti!();
    };
}

macro_rules! impl_131 {
    () => {
        deps!();
        impl Drop for RawMulti { fn drop (& mut self) { unsafe { let _ = cvt (curl_sys :: curl_multi_cleanup (self . handle)) ; } } }
    };
}

impl_131!()