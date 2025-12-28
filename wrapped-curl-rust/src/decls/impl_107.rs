macro_rules! deps {
    () => {
        List!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        impl Drop for List { fn drop (& mut self) { unsafe { curl_sys :: curl_slist_free_all (self . raw) } } }
    };
}

impl_107!()