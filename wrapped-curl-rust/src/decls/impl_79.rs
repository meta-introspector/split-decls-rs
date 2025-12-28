macro_rules! deps {
    () => {
        Easy2!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl < H > Drop for Easy2 < H > { fn drop (& mut self) { unsafe { curl_sys :: curl_easy_cleanup (self . inner . handle) ; } } }
    };
}

impl_79!();