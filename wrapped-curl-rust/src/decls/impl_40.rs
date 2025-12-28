macro_rules! deps {
    () => {
        Form!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl Drop for Form { fn drop (& mut self) { unsafe { curl_sys :: curl_formfree (self . head) ; } } }
    };
}

impl_40!();